# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Relicensed from EUPL-1.2 to `MIT OR Apache-2.0`.
- Upgraded `syn` to version 3 and removed the unused direct `proc-macro2` dependency.
- The error now points at the offending field type instead of the whole struct.

### Added

- Support for tuple structs, unit structs and empty structs.
- Property tests (proptest), compile tests (trybuild) and mutation testing (cargo-mutants) in CI.

### Fixed

- The macro no longer panics on empty, unit or tuple structs.

## [0.1.1] - 2024-06-15

[0.1.1]: https://github.com/sunsided/ensure-uniform-type-rs//releases/tag/v0.1.1

### Added

- Added the `ensure_uniform_type` macro.
