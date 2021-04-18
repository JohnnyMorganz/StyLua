# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Main StyLua

To view the changelog of the StyLua binary, see [here](https://github.com/JohnnyMorganz/StyLua/blob/master/CHANGELOG.md)

## [Unreleased]

### Changed

- Improved internals to use VSCode file system

### Fixed

- Fixed issue where running the extension with bad/no internet caused it to fail whilst retrieving releases. We will now warn instead, and fall back to the installed version.

## [1.1.1] - 2021-02-25

### Fixed

- Fixed unicode characters affecting formatting range. We now convert from a character offset to a byte offset

## [1.1.0] - 2021-02-24

### Added

- Added support for formatting ranges rather than the whole document

## [1.0.3] - 2021-01-27

### Fixes

- Extension now handles bigger files better, previously it could cut them off
- StyLua binary can now be placed in a folder with spaces in it

## [1.0.2] - 2020-12-31

### Fixes

- Extension now honours `.styluaignore` files, and will not format a document if it is ignored
- Update icon and Extension information

## [1.0.1] - 2020-12-30

### Fixes

- Fix issue where extension would prompt an update was available but it was the same version

## [1.0.0] - 2020-12-30

- Initial release
