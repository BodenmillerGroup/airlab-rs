# airlab-rs

A tool to create [IMC](https://en.wikipedia.org/wiki/Mass_cytometry#Imaging_Mass_Cytometry_\(IMC\)) panels and manage the inventory.

![tests](https://github.com/BodenmillerGroup/airlab-rs/actions/workflows/test.yml/badge.svg)
![checks](https://github.com/BodenmillerGroup/airlab-rs/actions/workflows/check.yml/badge.svg)
![scheduled](https://github.com/BodenmillerGroup/airlab-rs/actions/workflows/scheduled.yml/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/airlab-web.svg)](https://crates.io/crates/airlab-web)

## Features

- **IMC panel creation**: build and export IMC panels.
- **Inventory**: Maintain the antibody and conjugate inventory.
- **Validations**: Maintain antibody/conjugate validations.

## Installation

You can install `airlab-web` using Cargo:

```sh
cargo install airlab-web
```

## Requirements

To run airlab, you need access to a PostgreSQL database. You can find various options to install PostgreSQL [here](https://www.postgresql.org/download/).

## Usage

We recommend that you set these env variables using a "infrastructure as code" tool such as [ansible](https://en.wikipedia.org/wiki/Ansible_\(software\))

```sh
export SERVICE_PWD_KEY="<KEY>"
export SERVICE_TOKEN_KEY="<TOKEN>"
export SERVICE_DB_URL="postgres://<db_user>:<db_password>@<db_host>/<db_name>"
export SERVICE_TOKEN_DURATION_SEC="36000"
export SERVICE_WEB_FOLDER="web-airlab/"
export RUST_LOG="airlab_web=warn,airlab_lib=warn"
export SERVICE_EMAIL_FROM_ADDRESS="<from_email_address>"
export SERVICE_EMAIL_FROM_NAME="<from_email_name>"
export SERVICE_EMAIL_TOKEN="<email_token>"
export SERVICE_EMAIL_ADDRESS="<email_service_url>"
export SERVICE_LOG_AGGR_URL="<log aggregator url>"
export SERVICE_RESET_PWD_URL="<reset password url>"
export SERVICE_HOST_ADDR="<host address>"
export SERVICE_HOST_PORT="<port>"
export SERVICE_DATA_PATH="data/airlab-data"
export SUPER_USER="<airlab super user email>"
export SUPER_USER_PWD="<airlab super user password>"
export SETUP_DEMO_GROUP="false"
airlab-web
```

- The super user is the intial user that can set up groups and manage users.
- If setup demo group is set to true, a demo group with a small panel is created.

## Contributing

We welcome contributions! Please check out our [CONTRIBUTING.md](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Make your changes and commit them (`git commit -m "Add feature"`)
4. Push to your branch (`git push origin feature-branch`)
5. Open a Pull Request

## Code of Conduct

This project follows our [Code of Conduct](CODE_OF_CONDUCT.md). Please read and adhere to the guidelines when contributing.

## License

This project is licensed under the [MIT License](LICENSE). See the `LICENSE` file for details.

## Acknowledgements

This project is developed and maintained by [Lars Malmstroem](mailto:lars.malmstroem@uzh.ch) and collaborators at University of Zurich.


---

For more details, visit our documentation at [Docs.rs](https://docs.rs/airlab-lib/latest/airlab_lib/) or our [GitHub repository](https://github.com/BodenmillerGroup/airlab-rs).
