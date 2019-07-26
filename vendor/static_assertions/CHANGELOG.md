# Changelog [![Crates.io][crate-badge]][crate]
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

## [0.2.5] - 2017-12-12
### Changed
- `assert_eq_size_ptr` wraps its code inside of a closure, ensuring that the
unsafe code inside never runs
- Clippy no longer warns about `unneeded_field_pattern` within `assert_fields`

### Added
- Much better documentation with test examples that are guaranteed to fail at
compile-time

### Removed
- Removed testing features; compile failure tests are now done via doc tests

## [0.2.4] - 2017-12-11
### Removed
- Removed the actual call to `mem::transmute` while still utilizing it for size
verification ([Simon Sapin], [#5])

### Added
- `assert_cfg` macro that asserts that the given configuration is set
- `assert_fields` macro to assert that a struct type or enum variant has a given
field

### Fixed
- Allow more generics flexibility in `assert_impl`

## [0.2.3] - 2017-08-24
### Fixed
- Trailing commas are now allowed

### Removed
- Removed clippy warnings

## [0.2.2] - 2017-08-13
### Added
- Added `assert_impl` macro to ensure a type implements a given set of traits

## [0.2.1] - 2017-08-13
### Added
- Added `assert_obj_safe` macro for ensuring that a trait is object-safe

## [0.2.0] - 2017-08-12
### Added
- Added `assert_eq_size_ptr` macro

### Fixed
- Allow `assert_eq_size`, `const_assert`, and `const_assert_eq` in non-function
contexts via providing a unique label [#1]

### Removed
- **[Breaking]** Semicolon-separated `assert_eq_size` is no longer allowed

## [0.1.1] - 2017-08-12
### Added
- Added `const_assert_eq` macro

## 0.1.0 - 2017-08-12

Initial release

[Simon Sapin]: https://github.com/SimonSapin

[#1]: https://github.com/nvzqz/static-assertions-rs/issues/1
[#5]: https://github.com/nvzqz/static-assertions-rs/pull/5

[crate]:       https://crates.io/crates/static_assertions
[crate-badge]: https://img.shields.io/crates/v/static_assertions.svg

[Keep a Changelog]:    http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

[Unreleased]: https://github.com/nvzqz/static-assertions-rs/compare/v0.2.5...HEAD
[0.2.5]: https://github.com/nvzqz/static-assertions-rs/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/nvzqz/static-assertions-rs/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/nvzqz/static-assertions-rs/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/nvzqz/static-assertions-rs/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/nvzqz/static-assertions-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/nvzqz/static-assertions-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/nvzqz/static-assertions-rs/compare/v0.1.0...v0.1.1
