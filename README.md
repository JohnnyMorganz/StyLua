<div align="center">
	<h1>
		StyLua<br>
		<a href="https://crates.io/crates/stylua"><img src="https://img.shields.io/crates/v/stylua.svg"></a>
    <a href="https://github.com/JohnnyMorganz/StyLua/actions/workflows/ci.yml"><img src="https://github.com/JohnnyMorganz/StyLua/actions/workflows/ci.yml/badge.svg"></a>
    <a href="https://codecov.io/gh/JohnnyMorganz/StyLua"><img src="https://codecov.io/gh/JohnnyMorganz/StyLua/branch/main/graph/badge.svg"/></a>
	</h1>
</div>

A deterministic code formatter for Lua 5.1, 5.2, 5.3, 5.4, LuaJIT, [Luau](https://luau.org/) and [CfxLua/FiveM Lua](https://docs.fivem.net/docs/scripting-manual/runtimes/lua/), built using [full-moon](https://github.com/Kampfkarren/full-moon).
StyLua is inspired by the likes of [prettier](https://github.com/prettier/prettier), it parses your Lua codebase, and prints it back out from scratch,
enforcing a consistent code style.

StyLua mainly follows the [Roblox Lua Style Guide](https://roblox.github.io/lua-style-guide/), with a few deviations.

## Installation

There are multiple ways to install StyLua:

### With Github Releases

Pre-built binaries are available on the [GitHub Releases Page](https://github.com/JohnnyMorganz/StyLua/releases).

By default, these are built with **all syntax variants enabled (Lua 5.2, 5.3, 5.4, LuaJIT and Luau)**, to cover all possible codebases.
See [configuring runtime syntax selection](#configuring-runtime-syntax-selection) if you need to select a particular syntax of Lua to format.
Alternatively, see [installing from crates.io](#from-cratesio) on how to install a particular flavour of StyLua.

### From Crates.io

If you have [Rust](https://www.rust-lang.org/) installed, you can install StyLua using cargo.
By default, this builds for just Lua 5.1.
You can pass the `--features <flag>` argument to add extra syntax variants:

```sh
cargo install stylua
cargo install stylua --features lua52
cargo install stylua --features lua53
cargo install stylua --features lua54
cargo install stylua --features luajit
cargo install stylua --features luau
```

You can specify multiple features at once, and then use [configuration in a `.stylua.toml` file](#configuring-runtime-syntax-selection) to defer syntax selection to runtime.

### GitHub Actions

The [stylua-action](https://github.com/marketplace/actions/stylua) GitHub Action can install and run StyLua.
This action uses the prebuilt GitHub release binaries, instead of running cargo install, for faster CI startup times.

### pre-commit

You can use StyLua with [pre-commit](https://pre-commit.com/).
There are 3 possible pre-commit hooks available:

- `stylua`: installs via cargo - requires the Rust toolchain
- `stylua-system`: runs a `stylua` binary available on the PATH. The binary must be pre-installed
- `stylua-github`: automatically installs the relevant prebuilt binary from GitHub Releases

Add the following to your `.pre-commit-config.yaml` file:

```yaml
- repo: https://github.com/JohnnyMorganz/StyLua
  rev: v2.1.0
  hooks:
    - id: stylua # or stylua-system / stylua-github
```

### npm

StyLua is available as a binary [published to npm](https://www.npmjs.com/package/@johnnymorganz/stylua-bin) as `@johnnymorganz/stylua-bin`.
This is a thin wrapper that installs the binary and makes it available through npm / npx.

```sh
npx @johnnymorganz/stylua-bin --help
```

StyLua is also available as a WASM library at [@johnnymorganz/stylua](https://www.npmjs.com/package/@johnnymorganz/stylua).
It is usable in Node.js, or in the browser (using a bundler).

### Docker

StyLua is available on the [Docker Hub](https://hub.docker.com/r/johnnymorganz/stylua).

If you are using Docker, the easiest way to install StyLua is:

```dockerfile
COPY --from=JohnnyMorganz/StyLua:2.1.0 /stylua /usr/bin/stylua
```

### Homebrew

StyLua is available on macOS via the [Homebrew](https://brew.sh) package manager.

```sh
brew install stylua
```

### Other Installation Methods

- [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.stylua)
- [Aftman](https://github.com/LPGhatguy/aftman)

```sh
aftman add johnnymorganz/stylua@2.1.0
```

- A community maintained package repository. Please note, these packages are maintained by third-parties and we do not control their packaging manifests.

[![Community Packages](https://repology.org/badge/vertical-allrepos/stylua.svg?header=Community%20Packages)](https://repology.org/project/stylua/versions)

### Other Editor Integrations

Note that these integrations require the StyLua binary to already be installed and available on your system.

- Sublime: [Sublime Text Package](https://github.com/aerobounce/Sublime-Pretty-Lua)
- Neovim: [stylua-nvim](https://github.com/ckipp01/stylua-nvim) / [stylua.nvim](https://github.com/wesleimp/stylua.nvim)
- Zed: [Zed Lua StyLua formatter settings](https://zed.dev/docs/languages/lua#stylua)

## Usage

Once installed, pass the files to format to the CLI:

```sh
stylua src/ foo.lua bar.lua
```

This command will format the `foo.lua` and `bar.lua` file, and search down the `src` directory to format any files within it.
StyLua can also read from stdin, by using `-` as the file name.

### Glob Filtering

By default, when searching through a directory, StyLua looks for all files matching the glob `**/*.lua` (or `**/*.luau` when `luau` is enabled) to format.
You can also specify an explicit glob pattern to match against when searching:

```sh
stylua --glob '**/*.luau' -- src # format all files in src matching **/*.luau
stylua -g '*.lua' -g '!*.spec.lua' -- . # format all Lua files except test files ending with `.spec.lua`
```

Note that the `-g/--glob` argument can take multiple strings at once, so `--` is required to separate between the glob patterns and the files to format.

By default, glob filtering (and `.styluaignore` files) are only applied during directory traversal and searching.
Files passed directly (e.g. `stylua foo.txt`) will override the glob / ignore and always be formatted.
To disable this behaviour, pass the `--respect-ignores` flag (`stylua --respect-ignores foo.txt`).

### Filtering using `.styluaignore`

You can create a `.styluaignore` file, with a format similar to `.gitignore`.
Any files matching the globs in the ignore file are ignored by StyLua.
For example, for a `.styluaignore` file with the following contents:

```
vendor/
```

running `stylua .` will ignore the `vendor/` directory.

### Filtering when using stdin

If you are formatting stdin by specifying `-` as the filename (usually as part of an editor integration)
you can optionally provide the filename via `--stdin-filepath`. To respect glob or `.styluaignore` filtering, pass `--respect-ignores`.

```stylua
stylua --respect-ignores --stdin-filepath src/foo.lua -
```

### `--check`: Checking files for formatting

To check whether files require formatting (but not write directly to them), use the `--check` flag.
It will take files as input, and output a diff to stdout instead of rewriting the file contents.
If there are any files that require formatting, StyLua will exit with status code 1.

There are different styles of output available:

- `--output-format=standard`: output a custom diff (default)
- `--output-format=unified`: output a unified diff, consumable by tools like `patch` or `delta`
- `--output-format=json`: output JSON representing the changes, useful for machine-readable output
- `--output-format=summary`: output a summary list of file paths that are incorrectly formatted

### `--verify`: Verifying formatting output

As a safety measure, you can use the `--verify` flag to verify the output of all formatting before saving the file.

If enabled, the tool will re-parse the formatted output to verify if the AST is still valid (no syntax errors) and is similar to the input (possible semantic changes).

This is useful when adopting StyLua in a large codebase, where it is difficult to manually check all formatting is correct.
Note that this may produce false positives and negatives - we recommend manual verification as well as running tests to confirm.

### Ignoring parts of a file

To skip formatting a particular part of a file, you can add `-- stylua: ignore` before it.
This is useful if there is a particular style you want to preseve for readability, e.g.:

```lua
-- stylua: ignore
local matrix = {
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
}
```

To skip a block of code, use `-- stylua: ignore start` and `-- stylua: ignore end`:

```lua
local foo = true
-- stylua: ignore start
local   bar   =   false
local  baz      = 0
-- stylua: ignore end
local foobar = false
```

Note that ignoring cannot cross scope boundaries - once a block is exited, formatting is re-enabled.

### Formatting Ranges

To format a specific range within a file, use `--range-start <num>` and/or `--range-end <num>`.
Both arguments are inclusive and optional - if an argument is not provided, the start/end of the file is used respectively.

Only whole statements lying within the range are formatted.
If part of a statement falls outside the range, the statement is ignored.

In editors, `Format Selection` is supported.

### Requires Sorting

StyLua has built-in support for sorting require statements. We group consecutive require statements into a single "block",
and then requires are sorted only within that block. Blocks of requires do not move around the file.

StyLua only considers requires of the form `local NAME = require(EXPR)`, and sorts lexicographically based on `NAME`.
(StyLua can also sort Roblox services of the form `local NAME = game:GetService(EXPR)`)

Requires sorting is off by default. To enable it, add the following to your `stylua.toml`:

```toml
[sort_requires]
enabled = true
```

## Configuration

StyLua has opinionated defaults, but also provides a few options that can be set per project.

### Finding the configuration

The CLI looks for a `stylua.toml` or `.stylua.toml` starting from the directory of the file being formatted.
It will keep searching upwards until it reaches the current directory where the tool was executed.
If not found, we search for an `.editorconfig` file, otherwise fall back to the default configuration.
This feature can be disabled using `--no-editorconfig`.
See [EditorConfig](https://editorconfig.org/) for more details.

Use `--config-path <path>` to provide a custom path to the configuration.
If the file provided is not found/malformed, StyLua will exit with an error.

By default, StyLua does not search further than the current directory.
Use `--search-parent-directories` to recursively search parent directories.
This will keep searching ancestors and, if not found, will then look in `$XDG_CONFIG_HOME` / `$XDG_CONFIG_HOME/stylua` / `$HOME/.config` and `$HOME/.config/stylua`.

**Note: enabling searching outside of the current directory is NOT recommended due to possibilities of conflicting formatting:**

It is recommended to keep a `.stylua.toml` file in your project root so that other developers can make use of the same configuration.

If a project uses the default configuration of StyLua without a configuration file present, enabling external searching may cause conflicting formatting.

### Configuring Runtime Syntax Selection

By default, StyLua releases comes with all flavours of Lua bundled into one binary, with a union of all syntax styles.
We do this to make it easier to get started with StyLua on any codebase or project using Lua.

However, there are times where the union of syntaxes collide, causing issues. For example, Lua 5.2's goto label syntax
(`::label::`) conflicts with Luau's type assertion syntax (`x :: number`), and the latter ends up taking priority.

To disambiguate a particular syntax style for your codebase, set `syntax = "Style"` in your `.stylua.toml` file, e.g.:

```toml
syntax = "Lua52"
```

Alternatively, you can specify it on the command line, with `stylua --syntax lua52 ...`

### Options

StyLua only offers the following options:

| Option                       | Default            | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ---------------------------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `syntax`                     | `All`              | Specify a disambiguation for the style of Lua syntax being formatted. Possible options: `All` (default), `Lua51`, `Lua52`, `Lua53`, `Lua54`, `LuaJIT`, `Luau`, `CfxLua`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `column_width`               | `120`              | Approximate line length for printing. Used as a guide for line wrapping - this is not a hard requirement: lines may fall under or over the limit.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `line_endings`               | `Unix`             | Line endings type. Possible options: `Unix` (LF) or `Windows` (CRLF)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `indent_type`                | `Tabs`             | Indent type. Possible options: `Tabs` or `Spaces`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `indent_width`               | `4`                | Character size of single indentation. If `indent_type` is set to `Tabs`, this option is used as a heuristic to determine column width only.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `quote_style`                | `AutoPreferDouble` | Quote style for string literals. Possible options: `AutoPreferDouble`, `AutoPreferSingle`, `ForceDouble`, `ForceSingle`. `AutoPrefer` styles will prefer the specified quote style, but fall back to the alternative if it has fewer string escapes. `Force` styles always use the specified style regardless of escapes.                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `call_parentheses`           | `Always`           | Whether parentheses should be applied on function calls with a single string/table argument. Possible options: `Always`, `NoSingleString`, `NoSingleTable`, `None`, `Input`. `Always` applies parentheses in all cases. `NoSingleString` omits parentheses on calls with a single string argument. Similarly, `NoSingleTable` omits parentheses on calls with a single table argument. `None` omits parentheses in both cases. Note: parentheses are still kept in situations where removal can lead to obscurity (e.g. `foo "bar".setup -> foo("bar").setup`, since the index is on the call result, not the string). `Input` removes all automation and preserves parentheses only if they were present in input code: consistency is not enforced. |
| `space_after_function_names` | `Never`            | Specify whether to add a space between the function name and parentheses. Possible options: `Never`, `Definitions`, `Calls`, or `Always`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `block_newline_gaps`         | `Never`            | Specify whether to preserve leading and trailing newline gaps for blocks. Possible options: `Never`, `Preserve`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| `collapse_simple_statement`  | `Never`            | Specify whether to collapse simple statements. Possible options: `Never`, `FunctionOnly`, `ConditionalOnly`, or `Always`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

Default `stylua.toml`, note you do not need to explicitly specify each option if you want to use the defaults:

```toml
syntax = "All"
column_width = 120
line_endings = "Unix"
indent_type = "Tabs"
indent_width = 4
quote_style = "AutoPreferDouble"
call_parentheses = "Always"
collapse_simple_statement = "Never"
space_after_function_names = "Never"
block_newline_gaps = "Never"

[sort_requires]
enabled = false
```
