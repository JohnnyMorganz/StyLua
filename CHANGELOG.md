# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Added support for removing excess parentheses around expressions.
e.g. `print((x))` will be formatted to `print(x)`, as the parentheses are unnecessary. We also consider cases
where parentheses should not be removed, e.g. `print((x()))` - removing the parentheses changes the meaning of the code.
- Added formatting of BinOp expressions within function calls. If there is a long expression as a function argument and it contains binops, it will now span multiple lines
- Added a `column_width` setting, which is used to guide when StyLua should wrap lines. It defaults to `120`.

### Changed
- Improved CLI `--check` output. We now use a more detailed output which should help in determining diffs
- Improved calculations in places to determine when to wrap lines

### Fixed
- Fixed an expression ending with an UnOp (e.g. `#foo`) and a trailing comment forcing an unnecessary hanging expression
- Fixed loss of comments trailing punctuation within function parameters
- Comments within function parameters now force the parameter to go mutliline, fixing syntax errors created from previous formatting
- Fixed incorrect indentation of body of expressions spanning multiple lines (e.g. anonymous functions/tables) when the expression is part of a hanging binop
- Fixed incorrect formatting of multiple long comma-separated assignment/returns causing the comma to be placed onto a new line

## [0.4.1] - 2021-02-05
### Fixed
- Fixed function calls being incorrectly expanded due to a comment within the arguments.
We will now only check for leading/trailing comments for argument expressions to see if we need to keep it expanded or not.

## [0.4.0] - 2021-02-05
### Added
- Added formatting for number literals which begin with a decimal. For consistency, a "0" will be prepended (i.e. `.5` turns to `0.5`)
- Long expressions in a return statement will now hang onto multiple lines if necessary
- StyLua will now handle expressions in parentheses if they are long, by breaking them down further.
- Added support for ambiguous syntax. StyLua will now keep the semicolon and format as required

### Fixed
- Fixed "then" and "do" tokens not being correctly indented when if-then and while-do statements are pushed onto multiple lines
- Fixed incorrect newline formatting when a return type is present for an anonymous function in Luau
- Fixed multiline expressions where the binop has a trailing comment being incorrectly formatted, breaking code
- Fixed a trailing comment at the end of a whole binop expression unnecessarily forcing a hanging expression

## [0.3.0] - 2021-01-15
### Added
- StyLua will now test escapes of characters other than quotes in strings to see if they are unnecessary and remove them if so
- Adds wrapping for large expressions to push them onto multiple lines. Statements with line of longer than 120 characters will trigger expression wrapping where possible.
The expression will be split at its Binary Operators, excluding relational operators.

### Fixed
- Fixed `.styluaignore` file extension matching not working due to the default override glob
- Cleaned up descriptions of options when running `stylua --help`
- Fixed issue with `stylua.toml` requiring a complete configuration file with all options set
- Fixed issue with escapes unrelated to quotes inside of strings not being preserved
- Fixed incorrect formatting when trailing comments are present in function arguments and other locations.
In function arguments, it will remain expanded if there is a comment present. Similarly, comments are now preserved in punctuated sequencues.

## [0.2.1] - 2021-01-03
### Fixed
- Fixed `until` token in a repeat block not being correctly indented
- Fixed regression causing the first and last item of an expanded table to not be correctly indented

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