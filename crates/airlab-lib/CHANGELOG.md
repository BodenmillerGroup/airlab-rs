# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Merged private dev branch d01 ([`c5ac8af`](https://github.com/BodenmillerGroup/airlab-rs/commit/c5ac8afff5317ff21fc8ebf7d0b9150be86ae9f3))
</details>

