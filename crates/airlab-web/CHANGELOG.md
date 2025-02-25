

## v0.1.1 (2025-02-25)

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

 - 3 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release airlab-web v0.1.1 ([`458b1ca`](https://github.com/BodenmillerGroup/airlab-rs/commit/458b1ca820a976d7c88cd5aa07ab98ff40281666))
    - Unknown db user ([`6bb54fe`](https://github.com/BodenmillerGroup/airlab-rs/commit/6bb54fe588c915d56752ec84c1719f1bde9cbd77))
    - Merged private dev branch d01 ([`c5ac8af`](https://github.com/BodenmillerGroup/airlab-rs/commit/c5ac8afff5317ff21fc8ebf7d0b9150be86ae9f3))
</details>

