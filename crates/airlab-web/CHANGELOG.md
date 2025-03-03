

## v0.1.3 (2025-03-03)

### Documentation

 - <csr-id-3aa484a17915e71dc7599254f88e2a864791831e/> added the license to both crates
 - <csr-id-007c93072e0511b1878204444cf3595802b8ed7c/> added readme documentation

### New Features

 - <csr-id-af9d5ff046f60644502bf7fe434342b2e1832f4e/> upload and deleting validation slides
   - fix: handle missing files more gracefully fixes #5
   - feat: implemented delete validation_file
   - feat: upload validation file fixes #3
   - fix: warn if group has no member

### Bug Fixes

 - <csr-id-379df3ce81ea0ebd9e5312f0bdcb1c67325233c7/> added old tests; all ignored

### Other

 - <csr-id-e98bbfba4e84ca4a9287fd694e814b905903396b/> updated editions

### Style

 - <csr-id-3fd349bde95928ed4bbccf4bd8dc0375e589c9c4/> cargo fmt

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 7 commits contributed to the release over the course of 5 calendar days.
 - 6 days passed between releases.
 - 6 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release airlab-lib v0.1.3 ([`2d46cbf`](https://github.com/BodenmillerGroup/airlab-rs/commit/2d46cbf02ed0fb598bb4f0546894937bf68ffd6f))
    - Upload and deleting validation slides ([`af9d5ff`](https://github.com/BodenmillerGroup/airlab-rs/commit/af9d5ff046f60644502bf7fe434342b2e1832f4e))
    - Added old tests; all ignored ([`379df3c`](https://github.com/BodenmillerGroup/airlab-rs/commit/379df3ce81ea0ebd9e5312f0bdcb1c67325233c7))
    - Added the license to both crates ([`3aa484a`](https://github.com/BodenmillerGroup/airlab-rs/commit/3aa484a17915e71dc7599254f88e2a864791831e))
    - Cargo fmt ([`3fd349b`](https://github.com/BodenmillerGroup/airlab-rs/commit/3fd349bde95928ed4bbccf4bd8dc0375e589c9c4))
    - Added readme documentation ([`007c930`](https://github.com/BodenmillerGroup/airlab-rs/commit/007c93072e0511b1878204444cf3595802b8ed7c))
    - Updated editions ([`e98bbfb`](https://github.com/BodenmillerGroup/airlab-rs/commit/e98bbfba4e84ca4a9287fd694e814b905903396b))
</details>

## v0.1.2 (2025-02-25)

### New Features

 - <csr-id-c5ac8afff5317ff21fc8ebf7d0b9150be86ae9f3/> merged private dev branch d01
   - refactored from four crates to two
- added .github actions, modified from jonhoo/rust-ci-conf
- workflows: check.yml,scheduled.yml,test.yml
- added sqlx migrations
- added tests for model group
- added more tasks to the cargo make file
- added support to add a superuser at startup
- user creating the group is also the admin
- added support to create a demo group if wanted
- added tempates for .cargo/config.toml and .env
- bugfix: delete panel elements when removed by the user

### Bug Fixes

 - <csr-id-6bb54fe588c915d56752ec84c1719f1bde9cbd77/> unknown db user
   - stopped setting table ownership in 001_init.sql
- added env variables to the ci workflows

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 10 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release airlab-web v0.1.2 ([`6403fb5`](https://github.com/BodenmillerGroup/airlab-rs/commit/6403fb5e58d27aa58df4a45719c8a856b3c1fad8))
    - Release airlab-web v0.1.2 ([`abddc7e`](https://github.com/BodenmillerGroup/airlab-rs/commit/abddc7ebd96350ad632c01c3878379056006cc61))
    - Adjusting changelogs prior to release of airlab-lib v0.1.2, airlab-web v0.1.2 ([`21a8983`](https://github.com/BodenmillerGroup/airlab-rs/commit/21a898314b2336aa2d7cfc267e094e7d79c487ee))
    - Adjusting changelogs prior to release of airlab-lib v0.1.2, airlab-web v0.1.2 ([`3167d59`](https://github.com/BodenmillerGroup/airlab-rs/commit/3167d595de2653840a6e277ad0ec85d3618741c9))
    - Release airlab-lib v0.1.1, airlab-web v0.1.2 ([`f189df7`](https://github.com/BodenmillerGroup/airlab-rs/commit/f189df79a6e7450e974d39880d01f8b12e07e7f4))
    - Release airlab-lib v0.1.1, airlab-web v0.1.2 ([`93adb14`](https://github.com/BodenmillerGroup/airlab-rs/commit/93adb14c69c62bf609f792040330bc06b834ede5))
    - Release airlab-lib v0.1.1, airlab-web v0.1.1 ([`641aa2b`](https://github.com/BodenmillerGroup/airlab-rs/commit/641aa2bb06cf15906d57cbfaaf1b447873b85ff4))
    - Release airlab-web v0.1.1 ([`458b1ca`](https://github.com/BodenmillerGroup/airlab-rs/commit/458b1ca820a976d7c88cd5aa07ab98ff40281666))
    - Unknown db user ([`6bb54fe`](https://github.com/BodenmillerGroup/airlab-rs/commit/6bb54fe588c915d56752ec84c1719f1bde9cbd77))
    - Merged private dev branch d01 ([`c5ac8af`](https://github.com/BodenmillerGroup/airlab-rs/commit/c5ac8afff5317ff21fc8ebf7d0b9150be86ae9f3))
</details>

## v0.1.1 (2025-02-25)

### New Features

 - <csr-id-c5ac8afff5317ff21fc8ebf7d0b9150be86ae9f3/> merged private dev branch d01
   - refactored from four crates to two

### Bug Fixes

 - <csr-id-6bb54fe588c915d56752ec84c1719f1bde9cbd77/> unknown db user
   - stopped setting table ownership in 001_init.sql

<csr-unknown>
<csr-unknown>
<csr-unknown>
<csr-unknown>
added .github actions, modified from jonhoo/rust-ci-confworkflows: check.yml,scheduled.yml,test.ymladded sqlx migrationsadded tests for model groupadded more tasks to the cargo make fileadded support to add a superuser at startupuser creating the group is also the adminadded support to create a demo group if wantedadded tempates for .cargo/config.toml and .envbugfix: delete panel elements when removed by the useradded env variables to the ci workflows<csr-unknown/>
<csr-unknown/>
<csr-unknown/>
<csr-unknown/>

