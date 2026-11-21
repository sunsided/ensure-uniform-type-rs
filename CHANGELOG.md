# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0](https://github.com/sunsided/ensure-uniform-type-rs/compare/v0.1.1...v1.0.0) - 2026-11-21

### Added

- Add release-plz and tag-triggered crates.io publishing
- Add CONTRIBUTING.md
- Add git hooks and Copilot review rules
- Add REUSE licensing metadata
- Add supply-chain checks and declare MSRV 1.88
- Add codecov configuration
- Add Taskfile and hardened CI workflows
- Support tuple structs, upgrade to syn 3 and add tests

### Changed

- Prepare the 1.0.0 release
- Enforce pedantic lints and forbid unsafe code
- Leave changelog entries to release-plz
- Move to edition 2024
- Use the README as crate documentation
- Relicense under MIT OR Apache-2.0

### Removed

- Drop semver checks for this proc-macro crate

## [0.1.1] - 2024-06-15

[0.1.1]: https://github.com/sunsided/ensure-uniform-type-rs//releases/tag/v0.1.1

### Added

- Added the `ensure_uniform_type` macro.
