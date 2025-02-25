

## v0.1.1 (2025-02-25)

### New Features

 - <csr-id-c5ac8afff5317ff21fc8ebf7d0b9150be86ae9f3/> merged private dev branch d01
   - refactored from four crates to two

### Bug Fixes

 - <csr-id-6bb54fe588c915d56752ec84c1719f1bde9cbd77/> unknown db user
   - stopped setting table ownership in 001_init.sql

<csr-unknown>
added .github actions, modified from jonhoo/rust-ci-confworkflows: check.yml,scheduled.yml,test.ymladded sqlx migrationsadded tests for model groupadded more tasks to the cargo make fileadded support to add a superuser at startupuser creating the group is also the adminadded support to create a demo group if wantedadded tempates for .cargo/config.toml and .envbugfix: delete panel elements when removed by the useradded env variables to the ci workflows<csr-unknown/>

