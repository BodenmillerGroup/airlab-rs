# airlab-rs

`airlab-rs` is a rewrite of <https://github.com/BodenmillerGroup/airlab-web>.

## GitHub Actions

This repository currently uses two GitHub Actions workflows:

- `.github/workflows/ci.yml`
  Runs on push to `main` and `master`, on pull requests, and on manual dispatch. It covers:
  - frontend install, build, and tests
  - Rust formatting, clippy, `cargo audit`, and `cargo deny`
  - Rust tests against PostgreSQL
  - optional Playwright E2E execution when manually requested

- `.github/workflows/release.yml`
  Runs when a matching tag is pushed:
  - `v*`
  - `airlab-lib-v*`
  - `airlab-web-v*`

For `airlab-web`, the release workflow builds and publishes release assets. A concrete example is:

- <https://github.com/BodenmillerGroup/airlab-rs/releases/tag/airlab-web-v0.2.0>

That release workflow currently produces artifacts such as:

- `airlab-web-linux-x64-<tag>`
- `airlab-web-frontend-dist-<tag>.tar.gz`
- `airlab-web-migrations-<tag>.tar.gz`

## Releasing `airlab-web`

One practical release flow for `airlab-web` is:

1. Update the crate versions in both Cargo manifests:
   - `crates/airlab-lib/Cargo.toml`
   - `crates/airlab-web/Cargo.toml`
2. Commit the version bump.
3. Run:

```bash
cargo smart-release airlab-web -b minor --execute --no-publish
```

This is a good fit when you want `smart-release` to manage the release commit and the tag creation, but you do not want to publish crates to `crates.io`.

For this repository, the important practical point is that the versions in the two crate manifests should be kept aligned for the release you are preparing:

- `crates/airlab-lib/Cargo.toml`
- `crates/airlab-web/Cargo.toml`

After the tag is pushed, GitHub Actions `release.yml` builds the Linux binary, the frontend distribution archive, and the migrations archive, then attaches them to the GitHub release.

## Example Deployment Layout

The following is one suggested deployment style for a Linux host using `systemd`. It is not the only supported deployment approach, but it is a concrete layout that matches the generated release artifacts well.

### Suggested filesystem layout

Expected paths to set up:

```bash
sudo tree -ug /etc/airlab/
sudo tree -ug /usr/share/airlab/web/
sudo tree -ug /var/lib/airlab
sudo ls -lath /usr/local/bin/airlab-web
sudo ls -lath /etc/systemd/system/airlab.service
```

Example result:

```text
[root     airlab  ]  /etc/airlab/
└── [root     airlab  ]  airlab.env

1 directory, 1 file
```

```text
[root     root    ]  /usr/share/airlab/web/
├── [root     root    ]  airlab.svg
├── [root     root    ]  assets
│   ├── [root     root    ]  Admin-CONwqaAf.js
│   ├── [root     root    ]  ClonesListView-Ckp-zy9H.js
│   ├── [root     root    ]  ...
│   └── [root     root    ]  validators-DiIqOHmi.js
├── [root     root    ]  index.html
└── [root     root    ]  vite.svg

2 directories, 116 files
```

```text
[airlab   airlab  ]  /var/lib/airlab

0 directories, 0 files
```

```text
-rwxr-xr-x 1 root root 21M Apr 14 19:07 /usr/local/bin/airlab-web
-rw-r--r-- 1 root root 829 Apr 13 03:37 /etc/systemd/system/airlab.service
```

### Suggested mapping of release artifacts

One straightforward deployment mapping is:

- install the release binary as `/usr/local/bin/airlab-web`
- unpack `airlab-web-frontend-dist-<tag>.tar.gz` into `/usr/share/airlab/web/`
- keep the SQL migration files from `airlab-web-migrations-<tag>.tar.gz` available with the deployed release
- keep runtime state in `/var/lib/airlab`
- keep environment configuration in `/etc/airlab/airlab.env`

`airlab-web` manages the database schema using `sqlx` migrations. The binary includes `sqlx::migrate!()` and applies the bundled migrations during startup, so the migration archive is part of the release payload and should stay aligned with the deployed binary version.

### Suggested ownership and permissions

For this deployment style, a sensible default is:

- `/usr/local/bin/airlab-web`: `root:root`, mode `0755`
- `/etc/systemd/system/airlab.service`: `root:root`, mode `0644`
- `/etc/airlab/airlab.env`: `root:airlab`, mode `0640`
- `/usr/share/airlab/web/`: `root:root`, directories `0755`, files `0644`
- `/var/lib/airlab`: `airlab:airlab` if the service needs to write there
- `/data/airlab-data`: writable by `airlab`, for example `airlab:airlab`

The main recommendation is:

- installed binaries and deployed frontend assets should normally be owned by `root`, not by the interactive admin user that copied them into place
- the `airlab` service account should own only the directories that must be writable at runtime

Example commands:

```bash
sudo chown root:root /usr/local/bin/airlab-web
sudo chmod 0755 /usr/local/bin/airlab-web

sudo chown root:root /etc/systemd/system/airlab.service
sudo chmod 0644 /etc/systemd/system/airlab.service

sudo chown root:airlab /etc/airlab/airlab.env
sudo chmod 0640 /etc/airlab/airlab.env

sudo chown -R root:root /usr/share/airlab/web
sudo find /usr/share/airlab/web -type d -exec chmod 0755 {} \\;
sudo find /usr/share/airlab/web -type f -exec chmod 0644 {} \\;

sudo chown -R airlab:airlab /var/lib/airlab
sudo chown -R airlab:airlab /data/airlab-data
```

### Suggested `systemd` unit

Example `/etc/systemd/system/airlab.service`:

```ini
[Unit]
Description=AirLab (Axum) backend
After=network-online.target data.mount postgresql.service
Requires=data.mount postgresql.service
Wants=network-online.target

[Service]
Type=simple
User=airlab
Group=airlab
ExecStart=/usr/local/bin/airlab-web
EnvironmentFile=-/etc/airlab/airlab.env
WorkingDirectory=/var/lib/airlab

# Only allow writes to /data (external volume)
ReadWritePaths=/data
ReadOnlyPaths=/etc/airlab /usr/share/airlab

NoNewPrivileges=yes
PrivateTmp=yes
ProtectSystem=full
ProtectHome=yes
ProtectKernelTunables=yes
ProtectKernelModules=yes
ProtectControlGroups=yes
LockPersonality=yes
MemoryDenyWriteExecute=yes
CapabilityBoundingSet=
AmbientCapabilities=

Restart=on-failure
RestartSec=2s

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

### Suggested environment file

Example `/etc/airlab/airlab.env`:

```dotenv
SERVICE_AIRLAB_WEB_FOLDER="NA"
SERVICE_HISTOCAT_WEB_FOLDER="NA"
SERVICE_PWD_KEY="<PWD>"
SERVICE_TOKEN_KEY="<TOKEN>"
SERVICE_DB_URL="<DB_URL>"
SERVICE_TOKEN_DURATION_SEC="36000"
SERVICE_WEB_FOLDER="/usr/share/airlab/web"
RUST_LOG="web_airlab=debug,lib_core=debug,lib_auth=debug,lib_utils=debug"
SERVICE_EMAIL_FROM_ADDRESS="<FROM_ADDRESS>"
SERVICE_EMAIL_FROM_NAME="<FROM_NAME>"
SERVICE_EMAIL_TOKEN="<EMAIL_TOKEN>"
SERVICE_EMAIL_ADDRESS="<EMAIL_URL>"
SERVICE_LOG_AGGR_URL="<LOG_AGGR_URL>"
SERVICE_LOG_AGGR_AUTH="<AGR_AUTH>"
SERVICE_LOG_AGGR_REQUEST_STREAM="airlab_requests"
SERVICE_LOG_AGGR_FRONTEND_STREAM="airlab_frontend"
SERVICE_LOG_AGGR_EVENT_STREAM="airlab_event"
SERVICE_NAME="airlab-web"
SERVICE_ENV="prod"
SERVICE_RESET_PWD_URL="<RESET_PWD_URL>"
SERVICE_HOST_ADDR="127.0.0.1"
SERVICE_HOST_PORT="9080"
SERVICE_DATA_PATH="/data/airlab-data"
SUPER_USER="admin@example.com"
SUPER_USER_PWD="changeit"
SETUP_DEMO_GROUP="false"
```

In this deployment style:

- static frontend files are served from `/usr/share/airlab/web`
- the binary lives at `/usr/local/bin/airlab-web`
- runtime working directory is `/var/lib/airlab`
- mutable application data lives under `/data/airlab-data`
- configuration is injected through `/etc/airlab/airlab.env`

## Authors

- [Lars Malmstroem](mailto:lars.malmstroem@uzh.ch) (main author)

## Attribution

- design inspired by <https://github.com/rust10x/rust-web-app>

## Service Operations

For a `systemd` deployment, a minimal operational workflow is:

```bash
sudo systemctl start airlab
sudo systemctl stop airlab
sudo systemctl restart airlab
sudo systemctl status airlab
```

Useful log examples:

```bash
sudo journalctl -u airlab -n 100
sudo journalctl -u airlab -f
sudo journalctl -u airlab --since "1 hour ago"
```
