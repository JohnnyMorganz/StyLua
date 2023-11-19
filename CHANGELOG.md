# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- Fixed handling of floor division (`//`) syntax when only Luau FFlag is enabled
- Fixed missing space when table is inside of Luau interpolated string expression (`{{` is invalid syntax)

## [0.19.1] - 2023-11-15

This release has no changes. It resolves an issue in our test suite that may affect downstream package management tooling
failing tests ([#824](https://github.com/JohnnyMorganz/StyLua/issues/824))

## [0.19.0] - 2023-11-12

### Added

- Added flag `--respect-ignores`. By default, files explicitly passed to stylua (e.g. `stylua foo.lua`) will always be formatted, regardless of whether the file is ignored. Enabling this flag will consider `.styluaignore` or glob matches before formatting the file. ([#765](https://github.com/JohnnyMorganz/StyLua/issues/765))
  - Note: for backwards compatibility reasons, formatting via stdin always respects ignores. This behaviour will change in the next major release

### Changed

- Updated parser crate with following changes:

  - Support Luau floor division (`//`)
  - Fix Luau string interpolation parsing
  - Fix Luau `\z` escape parsing

- Simplified access and modification patterns for StyLua configuration. You can now access the properties directly

  - **Deprecated:** the old access patterns of `.property()` and `.with_property()` are now deprecated
  - **Breaking Change (WASM):** due to JS/TS lack of differentiation between `.property` / `.property()` implementation, the `.property()` functions were removed from WASM output.

- Multiline comments before commas will now remain in place and not move to after the comma. This is to support type-assertions-via-comments that is commonly used by some language servers. ([#778](https://github.com/JohnnyMorganz/StyLua/issues/778))

### Fixed

- Wasm build now correctly supports configuring sort requires ([#818](https://github.com/JohnnyMorganz/StyLua/issues/818))

## [0.18.2] - 2023-09-10

### Fixed

- Fixed LuaJIT suffixes `LL`/`ULL` causing a panic when running in `--verify` mode ([#750](https://github.com/JohnnyMorganz/StyLua/issues/750))
- Fixed incorrect formatting of conditionals when `collapse_simple_statement` is enabled and the block begins with an empty line ([#744](https://github.com/JohnnyMorganz/StyLua/issues/744))
- Fixed formatting of dot function call chains with comment between dot and names ([#747](https://github.com/JohnnyMorganz/StyLua/issues/747))

## [0.18.1] - 2023-07-15

### Fixed

- Fixed parentheses around a single Luau type pack in a generic being removed causing syntax errors ([#729](https://github.com/JohnnyMorganz/StyLua/issues/729))

## [0.18.0] - 2023-06-14

### Added

- Multiline ignores (`-- stylua: ignore start` / `-- stylua: ignore end`) will now work within table fields ([#705](https://github.com/JohnnyMorganz/StyLua/issues/705)):

```lua
require("foo").bar {
	-- stylua: ignore start
	baz      =0, -- < not formatted
	foo   =   2, -- < not formatted
	-- stylua: ignore end
	bar        =     1234 -- formatted
}
```

- Added option `"Input"` to `call_parentheses` setting, where call parentheses are retained based on their presence in the original input code. ([#668](https://github.com/JohnnyMorganz/StyLua/issues/668))
  Note: this setting removes all automation in determining call parentheses, and consistency is not enforced.

### Changed

- Improved heuristics around Luau type excess parentheses removal, so unnecessary types are removed in more locations

### Fixed

- Function calls are now formatted onto multiple lines if the opening brace `{` of a multiline table forces one of the lines over width ([#704](https://github.com/JohnnyMorganz/StyLua/issues/704))
- Fixed missing option `--sort-requires` to enable sort requires on the command line ([#669](https://github.com/JohnnyMorganz/StyLua/issues/669))

```sh
$ stylua --sort-requires test.lua
```

- Fixed parentheses removed around Luau optional type `(B?)` causing syntax errors when present in an intersection `A & (B?)` ([#679](https://github.com/JohnnyMorganz/StyLua/issues/679))
- Fixed comments lost when parentheses removed around Luau types
- Fixed race condition where if a file is passed more than once as an argument to format, then it could potentially be wiped completely (for example, if an ancestor directory is passed and recursively searched, as well as the file itself) ([#708](https://github.com/JohnnyMorganz/StyLua/issues/708))

## [0.17.1] - 2023-03-30

### Fixed

- Bumped internal parser dependency which should fix parsing problems for comments with Chinese characters, and multiline string escapes
- Fixed comments in punctuated lists for return statements or assignments being incorrectly formatted leading to syntax errors ([#662](https://github.com/JohnnyMorganz/StyLua/issues/662))
- Fixed line endings not being correctly formatted in multiline string literals and comments ([#665](https://github.com/JohnnyMorganz/StyLua/issues/665))

## [0.17.0] - 2023-03-11

### Added

- Added support for "sort requires", which sorts top-level statements of the form `local NAME = require(EXPR)` lexicographically on `NAME`.
  We do this by treating a group of consecutive requires as a "block", and then sorting **only within** the block. Any other statement, or an empty line, between require statements will split the group into two separate blocks (and can be used to separate the sorting). A block of requires will not move around the file.
  Roblox Luau statements of the form `local NAME = game:GetService(EXPR)` will also be sorted separately.

This feature is disabled by default. To enable it, add the following to your `stylua.toml`:

```toml
[sort_requires]
enabled = true
```

Note: we assume that all requires are **pure** with no side effects. It is not recommended to use this feature if the ordering of your requires matter.

- Added support for [EditorConfig](https://editorconfig.org/), which is taken into account only if no `stylua.toml` was found.

This feature is enabled by default, it can be disabled using `--no-editorconfig`.

- Published StyLua to the [Docker Hub](https://hub.docker.com/r/johnnymorganz/stylua)

## [0.16.1] - 2023-02-10

### Fixed

- Fixed mistransformation of a function argument in a multilined function call when the argument contains a comment, causing a syntax error. We now attempt to hang the expression ([#648](https://github.com/JohnnyMorganz/StyLua/issues/648))
- Fixed verify AST flagging a false positive for parentheses removed around a Luau type ([#643](https://github.com/JohnnyMorganz/StyLua/issues/643))

## [0.16.0] - 2023-01-15

### Added

- Unnecessary parentheses around Luau types will now be removed ([#611](https://github.com/JohnnyMorganz/StyLua/issues/611))
- Collapse a body containing only a `goto` statement when `collapse_simple_statement` is set ([#618](https://github.com/JohnnyMorganz/StyLua/issues/618))

### Changed

- Update internal parser:
  - (`lua52`) Support Lua 5.2 fractional hexidecimal / hexidecimal with exponents ([#621](https://github.com/JohnnyMorganz/StyLua/issues/621))
  - (`lua52`) Support LuaJIT number suffixes LL/ULL/i ([#621](https://github.com/JohnnyMorganz/StyLua/issues/621))
  - (`lua52`) Support `\z` escape sequences in strings ([#613](https://github.com/JohnnyMorganz/StyLua/issues/613))
  - (`luau`) Support Luau string interpolation ([#607](https://github.com/JohnnyMorganz/StyLua/issues/607))
- Several optimisations applied to formatting functions to reduce time taken. Files which previously did not terminate (6MB+) now finish in reasonable time. ([#591](https://github.com/JohnnyMorganz/StyLua/issues/591))
- Assignments of the form `local name = function` will no longer hang at the equals token, and instead force parameters multiline, to reduce unnecessary indentation. ([#595](https://github.com/JohnnyMorganz/StyLua/issues/595))

### Fixed

- Fixed an anonymous function assignment `local x = function()` being unnecessarily indented if the function body contains a comment ([#627](https://github.com/JohnnyMorganz/StyLua/issues/627))
- Fixed malformed formatting when there is a newline between a `return` token and the expressions ([#605](https://github.com/JohnnyMorganz/StyLua/issues/605))
- Fixed malformed formatting of multi-assignment or multi-returns where there is a comment within the expressions list ([#637](https://github.com/JohnnyMorganz/StyLua/issues/637))

## [0.15.3] - 2022-12-07

### Fixed

- Fixed necessary parentheses removed in `(-X) ^ Y` causing change in semantics ([#623](https://github.com/JohnnyMorganz/StyLua/issues/623))
- Take into account `function` token size when formatting an anonymous function `function() end` (particularly relevant when collapsing simple statements) ([#619](https://github.com/JohnnyMorganz/StyLua/issues/619))
- Support hanging inside of Luau type arrays `{ T }` to fix formatting issues when comments are present ([#617](https://github.com/JohnnyMorganz/StyLua/issues/617))

## [0.15.2] - 2022-10-31

### Fixed

- Fix incorrect indentation level used for hanging expressions in if expression syntax ([#596](https://github.com/JohnnyMorganz/StyLua/issues/596))
- Fixed Luau return type in parentheses containing a comment on the last item being collapsed causing a syntax error ([#608](https://github.com/JohnnyMorganz/StyLua/issues/608))
- Fix parentheses removed which highlight precedence in `(not X) == Y` causing linting errors ([#609](https://github.com/JohnnyMorganz/StyLua/issues/609))
- Fix build script for `@johnnymorganz/stylua` to include all lua and luau features ([#614](https://github.com/JohnnyMorganz/StyLua/issues/614))

## [0.15.1] - 2022-09-22

### Fixed

- Updated parser to fix comments parsing issues ([#585](https://github.com/JohnnyMorganz/StyLua/issues/585), [#587](https://github.com/JohnnyMorganz/StyLua/issues/587))

## [0.15.0] - 2022-09-21

### Added

- Added support for Lua 5.3, gated behind the `lua53` feature flag ([#534](https://github.com/JohnnyMorganz/StyLua/issues/534))
- Added support for Lua 5.4, gated behind the `lua54` feature flag ([#533](https://github.com/JohnnyMorganz/StyLua/issues/533))
- Added `--allow-hidden` flag to allow entering and formatting hidden files/directories ([#562](https://github.com/JohnnyMorganz/StyLua/issues/562))
- Added `--output-format=summary` which can be used with `--check` to output a summary of the list of files not correctly formatted ([#573](https://github.com/JohnnyMorganz/StyLua/issues/573))

### Changed

- We will no longer expand function calls when it contains an inlined multiline comment ([#543](https://github.com/JohnnyMorganz/StyLua/issues/543), [#561](https://github.com/JohnnyMorganz/StyLua/issues/561))

### Fixed

- Precommit hook now supports downloading aarch64 binary for M1 macs ([#558](https://github.com/JohnnyMorganz/StyLua/issues/558))
- Fixed mistransformations of generic for loop with comments in the expression list ([#579](https://github.com/JohnnyMorganz/StyLua/issues/579))
- Fixed `then`/`else` token not taken into account when formatting an if-expression ([#582](https://github.com/JohnnyMorganz/StyLua/issues/582))

## [0.14.3] - 2022-08-27

### Fixed

- Fixed macOS aarch64 target in release workflow ([#528](https://github.com/JohnnyMorganz/StyLua/issues/528))
- Long union/interesection types inside of a parentheses will now cause the parentheses to expand multiline ([#531](https://github.com/JohnnyMorganz/StyLua/issues/531))
- Fixed leading comments lost from an expression when excessive parentheses are removed from it ([#530](https://github.com/JohnnyMorganz/StyLua/issues/530))
- Fixed comments present in a complex expression not forcing multiline hanging leading to a syntax error ([#524](https://github.com/JohnnyMorganz/StyLua/issues/524))
- Fixed unnecessary break on `else` in an if-expression when the expression contains a comment ([#520](https://github.com/JohnnyMorganz/StyLua/issues/520))
- Take into account the extra line created when hanging at equals token in an assignment. This should prevent unnecessary hanging ([#542](https://github.com/JohnnyMorganz/StyLua/issues/542))
- Fixed comments added to a newly created trailing comment not being formatted ([#547](https://github.com/JohnnyMorganz/StyLua/issues/547))
- Fixed call chain with a small prefix not being kept inlined causing unstable formatting ([#514](https://github.com/JohnnyMorganz/StyLua/issues/514))
- Fixed shape computation for table fields causing unnecessary expansion ([#551](https://github.com/JohnnyMorganz/StyLua/issues/551))
- Fixed hanging the prefix string in `("str"):call` unnecessarily when it provides no benefit ([#508](https://github.com/JohnnyMorganz/StyLua/issues/508))
- Fixed table field value being expanded when it could be hanged instead ([#541](https://github.com/JohnnyMorganz/StyLua/issues/541))

## [0.14.2] - 2022-07-27

### Fixed

- Fixed var expression with trailing comments on initial prefix being collapsed leading to malformed formatting ([#509](https://github.com/JohnnyMorganz/StyLua/issues/509))
- Fixed return with comment between return and expression being collapsed leading to malformed formatting ([#504](https://github.com/JohnnyMorganz/StyLua/issues/504))
- Fixed release assets for precommit by marking release artifacts as application/zip ([#496](https://github.com/JohnnyMorganz/StyLua/issues/496))

## [0.14.1] - 2022-07-21

### Changed

- Chained var expression formatting will now follow the exact same steps as chained function call formatting

### Fixed

- Fixed var expression with comments collapsing leading to malformed formatting ([#500](https://github.com/JohnnyMorganz/StyLua/issues/500))
- Fixed ignore behavior for `--stdin-filepath` ([#495](https://github.com/JohnnyMorganz/StyLua/issues/495))

## [0.14.0] - 2022-07-06

### Added

- `--output-format=json` now outputs all (error) messages in JSON format ([#453](https://github.com/JohnnyMorganz/StyLua/issues/453))
- Added WASM build support. Stylua is available on npm for consumption in Node.js or a browser (using a bundler) - https://www.npmjs.com/package/@johnnymorganz/stylua
- Ignore comments will now be respected before fields inside tables ([#448](https://github.com/JohnnyMorganz/StyLua/issues/448))
- Stylua library (`stylua_lib`) now exposes a `format_ast(ast, config, range, verification)` function to format a full-moon AST directly ([#482](https://github.com/JohnnyMorganz/StyLua/issues/482))
- Added `collapse_simple_statement` option. It can take the values `Never` (default), `FunctionOnly`, `ConditionalOnly` or `Always`. When enabled, "simple" functions or if statements (ones where they only return a value or have a simple statement such as a function call) will be collapsed onto a single line where possible.

### Changed

- We now attempt to first hang the equals token in an assignment before expanding the RHS expression, provided the expression is not "complex" ([#292](https://github.com/JohnnyMorganz/StyLua/issues/292), [#489](https://github.com/JohnnyMorganz/StyLua/issues/489))
- We now use the current indent level of comments preceding an `elseif`/`else` token to determine whether they should still be indented one level or inlined with the `elseif`/`else` token. ([#254](https://github.com/JohnnyMorganz/StyLua/issues/254))
- Static chained function calls (i.e., `foo.bar().baz()`) will now hang if necessary ([#368](https://github.com/JohnnyMorganz/StyLua/issues/368))
- The first call in a chained function call will now inline with the prefix if the prefix begins with an uppercase letter or the prefix is smaller (in length) than the indent width
- A chained function call will not expand if the first call gets inlined

### Fixed

- [**Luau**] Fixed spacing lost before a comment within a type generic ([#446](https://github.com/JohnnyMorganz/StyLua/issues/446))
- [**Luau**] Removed unnecessary expansion of a type generic with a single table as the parameter ([#442](https://github.com/JohnnyMorganz/StyLua/issues/442))
- Fixed incorrect extra indentation of an expanded parentheses passed as a function call argument ([#456](https://github.com/JohnnyMorganz/StyLua/issues/456))
- [**Luau**] Increased the shape size of the expression in a type assertion so that it will correctly hang if over width ([#466](https://github.com/JohnnyMorganz/StyLua/issues/466))
- Fixed binary expression in a table field containing a comment being collapsed leading to malformed formatted ([#471](https://github.com/JohnnyMorganz/StyLua/issues/471))
- Fixed end parentheses of a function call with a multiline comment internally being expanded onto a new line unnecessarily ([#473](https://github.com/JohnnyMorganz/StyLua/issues/473))
- Fixed severe performance regression with complex nested function calls ([#477](https://github.com/JohnnyMorganz/StyLua/issues/477))

## [0.13.1] - 2022-04-11

### Fixed

- Fixed leading trivia on semicolon lost when semicolon is removed ([#431](https://github.com/JohnnyMorganz/StyLua/issues/431))
- Fixed shape calculation of the RHS of a binary expression not correctly reset when hanging, causing it to expand unnecessarily ([#432](https://github.com/JohnnyMorganz/StyLua/issues/432))
- Fixed unstable formatting of tables at column width boundary ([#436](https://github.com/JohnnyMorganz/StyLua/issues/436))
- Fixed assignments no longer hanging at equals token if a comment is present, but the expression is not hangable at a binop. ([#439](https://github.com/JohnnyMorganz/StyLua/issues/439))
- Fixed unstable formatting around comments within type declarations ([#397](https://github.com/JohnnyMorganz/StyLua/issues/397), [#430](https://github.com/JohnnyMorganz/StyLua/issues/430))
- Fixed parentheses around type assertions in a binary expression being removed leading to incorrect semantics. ([#441](https://github.com/JohnnyMorganz/StyLua/issues/441))

## [0.13.0] - 2022-03-31

### Added

- Added support for alternative diff outputs. You can now use `--output-format=unified` or `--output-format=json` to output a unified diff or json mismatches list respectively. A unified diff can be fed into other tools such as `patch` or `delta`, whilst a JSON diff provides a more machine readable format useful for extensions. ([#230](https://github.com/JohnnyMorganz/StyLua/issues/230))

### Changed

- Migrate internal dependency for CLI arguments handling, with improved help messages.
- Type declarations consisting of unions/intersections where an inner type has a multiline comment will now force hanging
- Generic fors will no longer expand onto multiple lines if the expression looping over is a function call with a single table argument (e.g., `ipairs({ ... })`) ([#405](https://github.com/JohnnyMorganz/StyLua/issues/405))
- Excess parentheses around a type assertion will now be removed. ([#383](https://github.com/JohnnyMorganz/StyLua/issues/383), [[#425](https://github.com/JohnnyMorganz/StyLua/issues/425)])
- When hanging an assignment of an expression contained within parentheses, we do not add an extra indentation. The formatting is now consistent with expanded tables and function calls. ([#274](https://github.com/JohnnyMorganz/StyLua/issues/274))

### Fixed

- Fixed issue through static linking where Windows binary would not execute due to missing `VCRUNTIME140.dll`. ([#413](https://github.com/JohnnyMorganz/StyLua/issues/413))
- Fixed assignment with comment sometimes not hanging leading to malformed syntax. ([#416](https://github.com/JohnnyMorganz/StyLua/issues/416))
- Fixed block ignores not applied when multiple leading block ignore comments are present at once. ([#421](https://github.com/JohnnyMorganz/StyLua/issues/421))
- Fixed ordering of comments when semicolon after statement is removed. ([#423](https://github.com/JohnnyMorganz/StyLua/issues/423))

## [0.12.5] - 2022-03-08

### Fixed

- Fixed crashed due to unhandled generic type packs under the `luau` feature flag. ([#403](https://github.com/JohnnyMorganz/StyLua/issues/403))

## [0.12.4] - 2022-03-02

### Fixed

- Fixed long comments forcing unnecessary hanging of type declarations. ([#384](https://github.com/JohnnyMorganz/StyLua/issues/384))
- Fixed long intersection types not hanging. ([#382](https://github.com/JohnnyMorganz/StyLua/issues/382))
- Fixed comments being lost around a condition when unnecessary parentheses are removed. ([#389](https://github.com/JohnnyMorganz/StyLua/issues/389))
- Fixed multiline expression with comments inside parentheses being collapsed leading to a syntax error. ([#386](https://github.com/JohnnyMorganz/StyLua/issues/386))
- Fixed ignore comments not respected in child blocks of ignored statements. ([#387](https://github.com/JohnnyMorganz/StyLua/issues/387))
- Fixed values in type tables not hanging when over width. ([#394](https://github.com/JohnnyMorganz/StyLua/issues/394))
- Fixed type info generics not hanging when over width. ([#394](https://github.com/JohnnyMorganz/StyLua/issues/394))
- Fixed callback types with binop type parameters / return types not hanging leading to a syntax error when comments are present. ([#396](https://github.com/JohnnyMorganz/StyLua/issues/396))
- Fixed type declarations not hanging properly causing them to go over width. This includes hanging at the equals token and hanging union/intersection types.

## [0.12.3] - 2022-02-17

### Fixed

- Fixed call chains not hanging when comments were present in between calls, leading to a syntax error. ([#367](https://github.com/JohnnyMorganz/StyLua/issues/367))
- Fixed if-expression syntax getting unnecessarily expanded further due to trailing comments. ([#375](https://github.com/JohnnyMorganz/StyLua/issues/375))
- Fixed formatting of leading comments of a keyword in if-expression syntax. ([#374](https://github.com/JohnnyMorganz/StyLua/issues/374))
- Fixed formatting of long type declarations which go over the line width to hang if possible. ([#372](https://github.com/JohnnyMorganz/StyLua/issues/372))
- Fixed mistransformation of comments within a type union leading to a syntax error. ([#378](https://github.com/JohnnyMorganz/StyLua/issues/378))

## [0.12.2] - 2022-02-06

### Fixed

- Fixed crash due to unhandled singleton type formatting under the `luau` feature flag. ([#358](https://github.com/JohnnyMorganz/StyLua/issues/358))
- Includes types in shape calculation for causing a generic for to go multiline under the `luau` feature flag. ([#360](https://github.com/JohnnyMorganz/StyLua/issues/360))

## [0.12.1] - 2022-02-01

### Fixed

- Fixed misformatting of conditions in if-expression syntax leading to spurious whitespace under the `luau` feature flag. ([#349](https://github.com/JohnnyMorganz/StyLua/issues/349))
- Fixed incorrect shape calculation in if-expression syntax: if-expression will now go multiline when only slightly over column width (`luau` feature flag).
- Fixed incorrect handling of comments at the end of a callback type's arguments under the `luau` feature flag. ([#352](https://github.com/JohnnyMorganz/StyLua/issues/352))
- Fixed mistransformation of type declaration when the type info is a union which must be multiline due to comments under the `luau` feature flag. ([#351](https://github.com/JohnnyMorganz/StyLua/issues/351))
- Fixed leading comments on a `|` symbol in a type info being lost when hanging the type under the `luau` feature flag.
- Fixed trailing comments of a function call being lost as parentheses are removed around a single argument when `call_parentheses` is set to not `Always`. ([#356](https://github.com/JohnnyMorganz/StyLua/issues/356))

## [0.12.0] - 2022-01-31

### Added

- Added option `call_parentheses`:
  Specify whether to apply parentheses on function calls with single string or table arg. Possible options: `Always` (default), `NoSingleString`, `NoSingleTable`, `None`. ([#329](https://github.com/JohnnyMorganz/StyLua/issues/329))
- Added proper multiline hanging of generic for syntax. ([#322](https://github.com/JohnnyMorganz/StyLua/issues/322))
- Added proper formatting for if-expression syntax under the `luau` feature flag. ([#289](https://github.com/JohnnyMorganz/StyLua/issues/289))
- Updated parser to add support for generic/variadic type packs, singleton types and default types under the `luau` feature flag.

### Fixed

- Fixed generic variadics not being handled under the `luau` feature flag. ([#333](https://github.com/JohnnyMorganz/StyLua/issues/333))
- Fixed issue with comments within an assignment not being correctly handled, leading to a syntax error. ([#340](https://github.com/JohnnyMorganz/StyLua/issues/340))
- Fixed parentheses around an IfExpression being removed, leading to incorrect semantics, under the `luau` feature flag. ([#345](https://github.com/JohnnyMorganz/StyLua/issues/345))

### Deprecated

- Option `no_call_parentheses` has been deprecated. Use `call_parentheses = "None"` instead.

## [0.11.3] - 2022-01-01

### Fixed

- Fixed comments preceding a comma within a function call or parameter list for a function definition being mistransformed leading to a syntax error. ([#307](https://github.com/JohnnyMorganz/StyLua/issues/307))
- Fixed IfExpression having abnormal leading whitespace under the `luau` feature flag. ([#315](https://github.com/JohnnyMorganz/StyLua/issues/315))
- Fixed incorrect handling of comments in unusual places within a table causing mistransformations leading to syntax errors. ([#318](https://github.com/JohnnyMorganz/StyLua/issues/318))

## [0.11.2] - 2021-11-15

### Fixed

- Fixed spaces around brackets string (`[[string]]`) used as an index or table key (i.e. `[ [[string]] ]`) being removed, leading to a syntax error. ([#293](https://github.com/JohnnyMorganz/StyLua/issues/293))
- Fixed incorrect shape calculation leading to arguments incorrectly expanding when under column width. ([#298](https://github.com/JohnnyMorganz/StyLua/issues/298))
- Fixed incorrect shape calculation for singleline table at the column width boundary. ([#296](https://github.com/JohnnyMorganz/StyLua/issues/296))
- Fixed IfExpression syntax containing extra/abnormal trailing whitespace when currently formatting as-is under the `luau` feature flag. ([#297](https://github.com/JohnnyMorganz/StyLua/issues/297))
- Fixed newlines before arguments in a function call which is later formatted on a single line being preserved, leading to inconsistent formatting. ([#290](https://github.com/JohnnyMorganz/StyLua/issues/290))
- Fixed odd formatting when returning multiple tables or functions only. ([#302](https://github.com/JohnnyMorganz/StyLua/issues/302))
- Fixed comments within an index expression (`foo[index]`) incorrectly handled leading to malformed formatting. ([#304](https://github.com/JohnnyMorganz/StyLua/issues/304))

## [0.11.1] - 2021-11-08

### Changed

- Updated internal parser to fix parsing issues and update `luau` parsing. ([#229](https://github.com/JohnnyMorganz/StyLua/issues/229), [#231](https://github.com/JohnnyMorganz/StyLua/issues/231))
- Default glob now matches `**/*.luau` (as well as `**/*.lua`) when the `luau` flag is enabled. ([#291](https://github.com/JohnnyMorganz/StyLua/issues/291))

### Fixed

- Fixed indentation of type callback specifier parameters when parameters have leading comment trivia. ([#278](https://github.com/JohnnyMorganz/StyLua/issues/278))
- Fixed trailing comma not being taken into account when determining the width of a field in a multiline table. ([#282](https://github.com/JohnnyMorganz/StyLua/issues/282))
- Fixed `--num-threads 1` causing a deadlock. ([#281](https://github.com/JohnnyMorganz/StyLua/issues/281))
- Fixed whitespace around parts of a binary expression causing it to over-hang in first pass, leading to unstable formatting. ([#287](https://github.com/JohnnyMorganz/StyLua/issues/287))

## [0.11.0] - 2021-09-16

### Changed

- In Luau type tables, a newline after the opening brace will now force the type table multiline. This is the same procedure as standard tables. ([#226](https://github.com/JohnnyMorganz/StyLua/issues/226))
- In Luau, type specifiers for function parameters will now force the parameters to be formatted multiline if a specifier is multiline (and there is more than one parameter).
- Improved error messages to make them easier to understand.

### Fixed

- Fixed range formatting no longer working when setting the range to statements inside nested blocks. ([#239](https://github.com/JohnnyMorganz/StyLua/issues/239))
- Fixed ignore file present in cwd not taken into account if cwd not included in file paths to format. ([#249](https://github.com/JohnnyMorganz/StyLua/issues/249))
- Fixed config locations (`$XDG_CONFIG_HOME` and `$HOME/.config`) not being looked into correctly on macOS when `--search-parent-directories` is used. ([#260](https://github.com/JohnnyMorganz/StyLua/issues/260))
- Fixed incorrect indentation of multiline type specifiers for function parameters under the `luau` feature flag. ([#256](https://github.com/JohnnyMorganz/StyLua/issues/256))
- Fixed unstable formatting caused by a singleline table which just reaches the column width. ([#261](https://github.com/JohnnyMorganz/StyLua/issues/261))
- Fixed misformatting of a binop expression as precedence of the RHS expression was not taken into account. ([#257](https://github.com/JohnnyMorganz/StyLua/issues/257), [#261](https://github.com/JohnnyMorganz/StyLua/issues/261))

## [0.10.1] - 2021-08-08

### Fixed

- Fixed an incorrect trailing comma being added to function args as part of a multiline expression list leading to a syntax error. ([#227](https://github.com/JohnnyMorganz/StyLua/issues/227))
- Fixed the first expression in a multiple assignment prematurely hanging even if its below the column width. ([#233](https://github.com/JohnnyMorganz/StyLua/issues/233))
- Updated internal parser to fix parsing issues for Luau code under the `luau` feature flag.

## [0.10.0] - 2021-07-11

### Added

- Added flag `--verify` which, when enabled, attempts to verify the generated output AST with the input AST to detect any changes to code correctness. Useful for adopting StyLua into a large codebase, at the cost of slower processing. ([#199](https://github.com/JohnnyMorganz/StyLua/issues/199))
- Added optional command line options `--column-width`, `--indent-type`, `--indent-width`, `--line-endings` and `--quote-style`, which, when provided, will override any configuration setting inferred from the default or a `stylua.toml`. ([#213](https://github.com/JohnnyMorganz/StyLua/issues/213))
- Added multithreaded support for formatting file in the CLI. Now each file will be formatted in its own thread. The number of threads used defaults to the number of cores on your computer, but can be set using `--num-threads`
- Added support for disabling formatting over specific ranges. Use `-- stylua: ignore start` to disable formatting and `-- stylua: ignore end` to re-enable it. The comment must be preceding a statement and disabling formatting cannot cross block scope boundaries. ([#198](https://github.com/JohnnyMorganz/StyLua/issues/198))

### Changed

- Luau type tables (`luau` feature flag) now use the same formatting strategy as normal expression tables, so that their formatting is more aligned.
- Luau typings now have improved checking against the current shape width to determine how to format if over column width.
- Luau callback types will now format multiline if they become over width under the `luau` feature flag.
- Improved the formatting of return expressions, they are now more in line with assignment expressions. ([#194](https://github.com/JohnnyMorganz/StyLua/issues/194))
- Changed buffering of error messages in the CLI. Originally, they would be buffered till the end, but now they are output immediately when seen.
- Allowed the use of `--check` when taking input from stdin.
- An error when parsing provided globs will cause the program to immediately exit rather than continuing with the incorrect glob.
- Only diff errors will exit with a status code of `1`. Other errors (e.g. parse errors or internal errors) will now exit with status code of `2`.

### Fixed

- Fixed comments inside Luau type tables leading to malformed formatting under the `luau` feature flag. ([#219](https://github.com/JohnnyMorganz/StyLua/issues/219))
- Fixed multiple assignment where an expression was originally hung due to comments being collapsed leading to malformed formatting. ([#222](https://github.com/JohnnyMorganz/StyLua/issues/222))
- Fixed an issue where a function call with a single table argument being hugged with the parentheses which contain comments leading to a syntax error. ([#224](https://github.com/JohnnyMorganz/StyLua/issues/224))

## [0.9.3] - 2021-06-26

### Added

- Added `--verbose` to print debug information, including finding config files and time taken to format files.

### Fixed

- Fixed severe performance regression due to a change in table formatting leading to exponential blowup for nested tables. ([#205](https://github.com/JohnnyMorganz/StyLua/issues/205))
- Fixed long binop chains with a comment deep inside not being hung, leading to a syntax error. ([#210](https://github.com/JohnnyMorganz/StyLua/issues/210))

## [0.9.2] - 2021-06-20

### Changed

- Bumped full-moon to `0.12.1` to fix parsing bugs

### Fixed

- Fixed parentheses around type assertions being classed as unnecessary and removed under the `luau` feature flag.
- Fixed mistransformation of function type where arguments have comments under the `luau` feature flag. ([#201](https://github.com/JohnnyMorganz/StyLua/issues/201))
- Fixed comments in an assignment in between the equals token and the expression leading to a mistransformation. ([#200](https://github.com/JohnnyMorganz/StyLua/issues/200))

## [0.9.1] - 2021-06-17

### Added

- Added `--stdin-filepath` option to specify location of file being taken in from stdin. This is optional and is only used to determine where to find the configuration file. If not provided, we default to searching from current working directory. ([#192](https://github.com/JohnnyMorganz/StyLua/issues/192))

### Fixed

- Fixed empty functions with comments being incorrectly collapsed leading to syntax error. ([#195](https://github.com/JohnnyMorganz/StyLua/issues/195))

## [0.9.0] - 2021-06-15

### Added

- CLI will now look for `stylua.toml` and its hidden counterpart, `.stylua.toml`. ([#145](https://github.com/JohnnyMorganz/StyLua/issues/145))
- Added CLI flag `--search-parent-directories`. If enabled, we will look in parent directories for a configuration file, or look in `$XDG_CONFIG_HOME` or `$XDG_CONFIG_HOME/stylua`. ([#127](https://github.com/JohnnyMorganz/StyLua/issues/127), [#146](https://github.com/JohnnyMorganz/StyLua/issues/146))
- Updated full-moon: Added support for typed variadics, named function type args, and generic functions under the Luau feature flag
- Will now hang on equality operators within binary expressions, if over width.
- If a file path is explicitly provided to the CLI which doesn't end with `.lua` ending, the `*.lua` glob check is skipped. ([#170](https://github.com/JohnnyMorganz/StyLua/issues/170))
- Long type unions will now hang under the `luau` feature flag. ([#165](https://github.com/JohnnyMorganz/StyLua/issues/165))
- Added option `no_call_parentheses`. Enabling this config will remove parentheses around function calls taking a single string/table as an argument. This config was added for adoption purposes. ([#133](https://github.com/JohnnyMorganz/StyLua/issues/133))

### Changed

- Long prefix expressions which are hangable and go over the line limit (e.g. `("foooo" .. "barrrrrrr" .. "bazzzzzz"):format(...)`) will now hang multiline ([#139](https://github.com/JohnnyMorganz/StyLua/issues/139))
- Changed formatting for assignments. We will now try all tactics then determine the best one. Multiple assignments will now no longer attempt to hang a single expression first - we will hang the whole punctuated list. ([#157](https://github.com/JohnnyMorganz/StyLua/issues/157))
- Function calls with single arguments are now possible to be expanded. This will allow the call to be expanded if the line goes over budget. ([#156](https://github.com/JohnnyMorganz/StyLua/issues/156))
- StyLua will now firstly prefer hanging long arguments to function calls to try and fit under the width, before expanding them multiline. ([#159](https://github.com/JohnnyMorganz/StyLua/issues/159))
- When hanging a binary expression, previously, we would always hang the "root" node of AST BinExp tree. Now we will check to see if is necessary (we are over width) before hanging ([#163](https://github.com/JohnnyMorganz/StyLua/issues/163))
- StyLua will hug together table braces with function call parentheses when formatting a function call taking a single table as an argument. ([#182](https://github.com/JohnnyMorganz/StyLua/issues/182))
- Function calls with more than one argument, where an argument is "complex", will now expand multiline. "complex" is an argument spanning multiple lines, but excludes a table or anonymous function, as we handle them explicitly. ([#183](https://github.com/JohnnyMorganz/StyLua/issues/183))
- StyLua will always hang at the equals token for a multi-variable assignment. ([#185](https://github.com/JohnnyMorganz/StyLua/issues/185))
- Tables with multiline fields (such as an anonymous function expression) should always expand if previously on single line. ([#187](https://github.com/JohnnyMorganz/StyLua/issues/187))
- Function definitions (both normal and anonymous) with an empty body will now be kept on a single line. This is common for noop functions `local function noop() end`. ([#188](https://github.com/JohnnyMorganz/StyLua/issues/188))

### Fixed

- Fixed 1 or 2 digit numerical escapes being incorrectly removed
- Fixed whitespace being lost before a multiline comment. We will now preserve a single space (e.g. `local test --[[foo]] = true` -> `local test --[[foo]] = true`) ([#136](https://github.com/JohnnyMorganz/StyLua/issues/136))
- Fixed the double formatting of a hanging call chain when it was being assigned to a variable causing it to be incorrectly formatted ([#151](https://github.com/JohnnyMorganz/StyLua/issues/151))
- Fixed leading comments to a binop in a hanging expression being lost ([#154](https://github.com/JohnnyMorganz/StyLua/issues/154#issuecomment-841703038))
- Fixed mistransformation of comments leading the RHS of a hanging binop. They are now moved to before the binop ([#154](https://github.com/JohnnyMorganz/StyLua/issues/154))
- Fixed comments trailing unnecessary parentheses around expressions that were later removed not being preserved ([#176](https://github.com/JohnnyMorganz/StyLua/issues/176))
- Fixed a double unary minus (`- -foo`/`-(-foo)`) being formatted as `--foo` leading to a comment syntax error. Parentheses are now enforced: `-(-foo)` ([#171](https://github.com/JohnnyMorganz/StyLua/issues/171))
- Fixed semicolon being removed leading to `function call x new statement` ambiguity when next statement is an assignment with the first variable being a parentheses var expression ([#173](https://github.com/JohnnyMorganz/StyLua/issues/173))
- Fixed mistransformation of comments in `if condition then` or `while condition do` lines - improved assurance that they will hang multiline ([#164](https://github.com/JohnnyMorganz/StyLua/issues/164))
- Fixed indentation of comments leading a `then` or `do` token when `if ... then` or `while ... do` are multiline.
- Fixed mistransformation of comments in a generic declaration under the `luau` feature flag ([#166](https://github.com/JohnnyMorganz/StyLua/issues/166))
- Fixed trailing comma being added after comments in multiline type table under the `luau` feature flag ([#166](https://github.com/JohnnyMorganz/StyLua/issues/166))

## [0.8.1] - 2021-04-30

### Fixed

- Fixed bug where a hanging expression inside of parentheses would lead to function arguments being incorrectly formatted with a trailing comma - leading to a syntax error

## [0.8.0] - 2021-04-30

### Added

- Parentheses around conditions are now removed, as they are not required in Lua. `if (foo and (not bar or baz)) then ... end` turns to `if foo and (not bar or baz) then ... end`
- Long multi-variable assignments which surpass the column width, even when hanging on the equals token, will now hang on multiple lines.

### Changed

- Changed the heursitics for when parentheses are removed around expressions. Parentheses will now never be removed around a function call prefix (e.g. `("hello"):len()`)
- Changed formatting for comma-separated lists. Previously, we would buffer the comments to the end of the list, but now we keep the comments next to where they original were.
- Improved contextual formatting informattion when formatting deep in the AST. We can now better determine how much space is left on the current line, before we need to change formatting
- Improved formatting of function declarations. It will now properly take into account the amount of space left on the column width.
- Improve formatting for assignments with expressions such as function calls. The whole assignment is now taken into account, so we can better determine whether to split the expression.

### Fixed

- Fixed trailing whitespace remaining on the last item of a multiline table (which was expanded from a singleline one)

## [0.7.1] - 2021-04-19

### Fixed

- Fixed parentheses around a table being incorrectly removed leading to a syntax error, such as in `({}):foo()`

## [0.7.0] - 2021-04-13

### Added

- Added hanging for chained function calls. See [#109](https://github.com/JohnnyMorganz/StyLua/issues/109)
- Long function definitions (normally with parameters containing types and a return type) will now be split across multiple lines if they surpass the column limit

### Changed

- Further improvements to the way binary expressions are hung on new lines

### Fixed

- Fixed trailing comments at the end of multiline tables being lost
- Fixed panic "stmt trailing comments not implemented" occuring due to incomplete function
- Fixed trailing comments after semicolons at the end of last statements being lost when formatting
- Fixed function parameters collapsing when there is a comments at the end of function parameters, where the last parameter has a type specifier
- Fixed comments at the end of tables being indented one extra level
- Fixed trailing comments within if-elseif-else blocks not being correctly indented.
- Fixed `do` in a `while ... do` statement not correctly indented when the condition spans multiple lines
- Fixed multiline parameters for a function definition inside of an indent block (e.g. a table) not being correctly indented

## [0.6.0] - 2021-03-27

### Added

- Added support for creating new `Config` structs when using StyLua as a library
- Added configuration for quote style. There are four quote style options - `AutoPreferDouble`, `AutoPreferSingle`, `ForceDouble` and `ForceSingle`.
  For the auto styles, we will prefer the quote type specified, but fall back to the opposite if it means there are fewer escapes. For the
  force styles, we will always use the quote type specified.
- StyLua will now error when unknown fields are found in the configuration `stylua.toml` file
- Long lines of assignments, where the expressions aren't hangable, will now be put onto a newline, where a newline is created after the equal sign, and the expressions indented.
- Added initial support for **Lua 5.2** syntax. StyLua can now format code containing `goto`s and labels. See [#87](https://github.com/JohnnyMorganz/StyLua/issues/87) to track further support for Lua 5.2 syntax.

### Changed

- Function call heuristic have been further improve to decide when to expand the function call arguments onto multiple lines.
- StyLua now allows some arguments after a multiline table before forcing expansion. This makes sense for something like `setmetatable({ ... }, class)`, where
  `{ ... }` is a multiline table, but we don't want to expand onto multiple lines. StyLua will not allow a mixture of multiline tables and small identifiers in between
  (e.g. `call({ ... }, foo, { ... })`), in order to improve readability.
- Empty newlines at the start and end of a block will now be removed as they are unnecessary
- Changed the default quote style from `ForceDouble` to `AutoPreferDouble`. We will now default to swapping quote type if it will reduce the number of escapes.

### Fixed

- Fixed tables with internal comments (and no fields) incorrectly collapsing to a single line
- Fixed parentheses being incorrectly removed around a BinOp where first value was a UnOp
- Fixed indentation of leading comments bound to the end brace of a multiline table
- Fixed LastStmt (return/break etc.) still being formatted when it wasn't defined inside the range
- Fixed hanging expressions which are inside function calls being indented unnecessarily by one extra level
- Fixed parentheses being incorrectly removed around a function definition, which may be called like `(function() ... end)()`
- Fixed some string escapes being incorrectly deemed as unnecessary
- Fixed trailing comments after semicolons at the end of statements being lost when formatting
- Fixed formatting issues in relation to newline and whitespace when using range formatting.
- Fixed empty tables taking 2 formatting passes to format properly

## [0.5.0] - 2021-02-24

### Added

- Added support for removing excess parentheses around expressions.
  e.g. `print((x))` will be formatted to `print(x)`, as the parentheses are unnecessary. We also consider cases
  where parentheses should not be removed, e.g. `print((x()))` - removing the parentheses changes the meaning of the code.
- Added formatting of BinOp expressions within function calls. If there is a long expression as a function argument and it contains binops, it will now span multiple lines
- Added a `column_width` setting, which is used to guide when StyLua should wrap lines. It defaults to `120`.
- Added support for formatting ranges. You can now specificy ranges using `--range-start <num>` and `--range-end <num>` (both optional, and both inclusive).
  If a range is provided, only statements within the range will be formatted. Currently only supports ranges containing whole statements, and is not more granular.
- Added support for ignore comments. If the line before a statement begins with the comment `-- stylua: ignore`, then the statement will be ignored during formatting.
  This currently only supports ignoring statement-level nodes

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

[unreleased]: https://github.com/JohnnyMorganz/StyLua/compare/v0.19.1...HEAD
[0.19.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.19.1
[0.19.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.19.0
[0.18.2]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.18.2
[0.18.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.18.1
[0.18.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.18.0
[0.17.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.17.1
[0.17.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.17.0
[0.16.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.16.1
[0.16.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.16.0
[0.15.3]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.15.3
[0.15.2]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.15.2
[0.15.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.15.1
[0.15.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.15.0
[0.14.3]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.14.3
[0.14.2]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.14.2
[0.14.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.14.1
[0.14.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.14.0
[0.13.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.13.1
[0.13.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.13.0
[0.12.5]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.12.5
[0.12.4]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.12.4
[0.12.3]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.12.3
[0.12.2]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.12.2
[0.12.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.12.1
[0.12.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.12.0
[0.11.3]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.11.3
[0.11.2]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.11.2
[0.11.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.11.1
[0.11.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.11.0
[0.10.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.10.1
[0.10.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.10.0
[0.9.3]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.9.3
[0.9.2]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.9.2
[0.9.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.9.1
[0.9.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.9.0
[0.8.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.8.1
[0.8.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.8.0
[0.7.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.7.1
[0.7.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.7.0
[0.6.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.6.0
[0.5.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.5.0
[0.4.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.4.1
[0.4.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.4.0
[0.3.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.3.0
[0.2.1]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.2.1
[0.2.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.2.0
[0.1.0]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.1.0
[0.1.0-alpha.3]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.1.0-alpha.3
[0.1.0-alpha.2]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.1.0-alpha.2
[0.1.0-alpha]: https://github.com/JohnnyMorganz/StyLua/releases/tag/v0.1.0-alpha
