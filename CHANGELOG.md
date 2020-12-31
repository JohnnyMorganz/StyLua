# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2020-12-31
### Changed
- Changed heuristics for expanding function arguments. StyLua will now check through the arguments and look out for expanded tables
or anonymous functions, and if found, will not expand the function call. However, if there are any other type of expression mixed between,
then the function call will remain expanded.
- Change internals of the formatter by reducing amount of cloning of AST nodes. Improves performance by 22%

## [0.1.0] - 2020-12-30
### Added
- StyLua will now take into account if a table was originally expanded onto multiple lines. If so, StyLua won't attempt to collapse it
- Added support for reading in from stdin for the CLI, use `stylua -` to make StyLua read from stdin, and a formatted output will be written to stdout
- Added `--check` command line flag. If enabled, then StyLua will check through the files and emit a diff for files with incorrect formatting, exiting with status code 1. StyLua will not modifiy files
- Renamed CLI argument `--pattern` to `--glob` (with short form `-g`). `--glob` can now accept multiple globs.
For example, using `stylua -g *.lua -g !*.spec.lua .` will format all Lua files apart from `.spec.lua` test files.
- Added support for parsing a `.styluaignore` file, which follows a similar structure to `.gitignore` files. Any patterns matched inside of this file will be ignored.

### Changed
- Changed when a table will expand onto new lines. It will now expand after 80 characters have been exceeded, and takes indent level into account

## [0.1.0-alpha.3] - 2020-12-26
### Changed
- Changed the default value of `indent_width` to 4
- Function calls with a single argument will no longer wrap the argument onto a new line. This is subject to change.

### Fixed
- Fixed a new line being added after the `until` token in a repeat block. The new line is now added at the end of the until expression.
- Fixed comments not being preserved within multiline tables
- Fixed trailing comma being added after comments in multiline tables
- Fixed escaping of double-quotes inside of strings being repeated
- Fixed long tables for types collapsing onto a single line for Luau formatting
- Fixed incorrect comment wrapping at the beginning of multiline tables
- Fixed start brace of multiline comments not having correct indentation
- Fixed comments having incorrect indentation when bound to the `end` token at the end of a block.

## [0.1.0-alpha.2] - 2020-12-22
### Added
- Single quote escapes are now removed from string literals if present when converting to double-quoted strings

### Changed
- If there is a single argument in a function call, and it is either a table or anonymous function, the relevant start/end tokens are no longer pushed onto new lines
- Comments are now left completely unformatted, apart from trimming trailing whitespace at the end of single-line comments

### Fixed
- Double quotes are now escaped when converting from single quote to double quote strings

## [0.1.0-alpha] - 2020-12-22
Initial alpha release