

## v0.1.4 (2026-03-17)

### New Features

 - <csr-id-baf7fb51460d1d430b4445876b63f67968fdaf6e/> release 26.03.16
   - more compact filter code and filter representation
   - panel elements, removed old code
   - refactored tag conj and added test
   - refactored group
   - refactored tags
   - validation files
   - refactored proteins
   - refactored species
   - refactored providers
   - refactor uses
   - member refactor
   - panel refactor
   - validation refactor
   - refactored lots
   - clones and conjugate refactors
   - ci using act working
   - new version of ratchet
   - disallow playwrigth to find the manual test file
   - more tests
   - vue tests
   - last tests
   - more tests
   - added more tests
   - added many tests
   - deploy bugs
 - <csr-id-ddf462834eea591af5f73a614f39ee1b30d685b8/> maintenance release
   - fixed bugs
   - refactor clones
   - new backend
   - fixed the backend for a few issues
   - starting with a real backend
   - more components migrated
   - mocks
   - added gen to find differneces
   - component migratinos
   - merged crosspane and airlab-rs
   - github actions workflow
 - <csr-id-7f82a2ae403ae73d7baf590f883822f5e2310e65/> maintenance release
   - removed warning
   - sorting in rust
   - filter on reactivity
   - removed unwrap and excepts
   - handle new filters
   - update the lot status from the conjugate view
   - collections filter for lots and conjugates
   - lot collection key
   - collection on lot
   - minor updates casuing warnings
   - fixed tag expansion and the conjugate menu
   - corrected several expansion menues
   - sorting in list views
   - support ordering
   - component updates to support paginagtion
   - store updates to support paginagtino
   - tests
   - protein test
   - lots pagintation
   - lots pagintation
   - update lots
 - <csr-id-e9674b815a667208ed1921fe7dcf562efdda4511/> storage and collections
   - re-order route
   - collections
   - collection backend
   - editing
   - update goes back to the list
   - updated helper to avoid quotations
   - added the storage id to conjugates
   - storage, and getting the active user to set as created_by for several items
   - added storage
   - storage drafted
   - sync data from the bblab server
   - refresh of the airlab-parseable strategy
   - more logging
   - placeholder tests
 - <csr-id-94232127c73c67eb59d136f9179fe24c71f49b56/> maintenance release
   - added validation routes
   - fixed validation
   - avoid naming collision
   - added filters to lots and conjugates
   - menu correctly alignted, proper status filer
   - ratchet v01
   - attempt to allow the telemetry data to be written on the bblab2 server
   - disabled sort for all except for lots
   - panel concentratinos
   - attempt to allow both filters and automatic path finders
   - attempt to reroute to login
   - build fixes
   - declared dead code
   - migrated the rust code

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 5 commits contributed to the release over the course of 32 calendar days.
 - 5 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release 26.03.16 (baf7fb5)
    - Maintenance release (ddf4628)
    - Maintenance release (7f82a2a)
    - Storage and collections (e9674b8)
    - Maintenance release (9423212)
</details>

## v0.1.3 (2025-03-03)

<csr-id-e98bbfba4e84ca4a9287fd694e814b905903396b/>
<csr-id-3fd349bde95928ed4bbccf4bd8dc0375e589c9c4/>

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

## v0.1.2 (2025-02-25)

### New Features

 - <csr-id-c5ac8afff5317ff21fc8ebf7d0b9150be86ae9f3/> merged private dev branch d01
   - refactored from four crates to two

### Bug Fixes

 - <csr-id-6bb54fe588c915d56752ec84c1719f1bde9cbd77/> unknown db user
   - stopped setting table ownership in 001_init.sql

<csr-unknown>
added .github actions, modified from jonhoo/rust-ci-confworkflows: check.yml,scheduled.yml,test.ymladded sqlx migrationsadded tests for model groupadded more tasks to the cargo make fileadded support to add a superuser at startupuser creating the group is also the adminadded support to create a demo group if wantedadded tempates for .cargo/config.toml and .envbugfix: delete panel elements when removed by the useradded env variables to the ci workflows<csr-unknown/>

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
<csr-unknown>
added .github actions, modified from jonhoo/rust-ci-confworkflows: check.yml,scheduled.yml,test.ymladded sqlx migrationsadded tests for model groupadded more tasks to the cargo make fileadded support to add a superuser at startupuser creating the group is also the adminadded support to create a demo group if wantedadded tempates for .cargo/config.toml and .envbugfix: delete panel elements when removed by the useradded env variables to the ci workflows<csr-unknown/>
<csr-unknown/>
<csr-unknown/>
<csr-unknown/>
<csr-unknown/>

