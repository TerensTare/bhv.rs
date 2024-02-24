
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2024-02-24

### Added

- Experimental event-based running of nodes under the `events` feature. See [README](README.md#events) for more.


## [0.2.0] - 2023-10-24

### Added

- Builder methods `repeat_until_pass` and `repeat_until_fail` to repeat a behavior until it passes or fails, respectively.
- [README](README.md) containing an introduction to the crate.

### Changed

- For documentation purposes, the `BhvExt` crate is exported as an actual name.
- `Bhv::execute` now accepts the context by value instead of by mutable reference.
- Rewrote the documentation to make it more natural.
- Reduced the visibility of members of some decorator nodes to forbid construction these nodes directly. Use `BhvExt` to construct these nodes instead.
- Added more fields to `Cargo.toml`.

### Fixed

- Bug where composite behaviors (sequences and selectors) wouldn't reset to their original state after they were completed.


## [0.1.2] - 2023-10-23

### Changed

- Improved documentation.
- Rewrote tests into inline doc tests.
- Reduced visibility scope of certain items to avoid leaking them out of the crate.

## [0.1.0] - 2023-10-23

- Initial release.