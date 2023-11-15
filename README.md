<div align="center">
	<h1>
		StyLua<br>
		<a href="https://crates.io/crates/stylua"><img src="https://img.shields.io/crates/v/stylua.svg"></a>
    <a href="https://github.com/JohnnyMorganz/StyLua/actions/workflows/ci.yml"><img src="https://github.com/JohnnyMorganz/StyLua/actions/workflows/ci.yml/badge.svg"></a>
    <a href="https://codecov.io/gh/JohnnyMorganz/StyLua"><img src="https://codecov.io/gh/JohnnyMorganz/StyLua/branch/main/graph/badge.svg"/></a>
	</h1>
</div>

An opinionated code formatter for Lua 5.1, 5.2, 5.3, 5.4 and [Luau](https://roblox.github.io/luau/), built using [full-moon](https://github.com/Kampfkarren/full-moon).
StyLua is inspired by the likes of [prettier](https://github.com/prettier/prettier), it parses your Lua codebase, and prints it back out from scratch,
enforcing a consistent code style.

StyLua mainly follows the [Roblox Lua Style Guide](https://roblox.github.io/lua-style-guide/), with a few deviations.

## Installation

There are multiple ways to install StyLua:

### With Github Releases

Pre-built binaries are available on the [GitHub Releases Page](https://github.com/JohnnyMorganz/StyLua/releases).

By default, these are built with **all syntax variants enabled (Lua 5.2, 5.3, 5.4 and Luau)**, to cover all possible codebases.
If you would like to format a specific Lua version only, see [installing from crates.io](#from-cratesio).

### From Crates.io

If you have [Rust](https://www.rust-lang.org/) installed, you can install StyLua using cargo.
By default, this builds for just Lua 5.1.
You can pass the `--features <flag>` argument to build for Lua 5.2 (`lua52`), Lua 5.3 (`lua53`), Lua 5.4 (`lua54`) or Luau (`luau`)

```sh
cargo install stylua
cargo install stylua --features lua52
cargo install stylua --features lua53
cargo install stylua --features lua54
cargo install stylua --features luau
```

### GitHub Actions

You can use the [stylua-action](https://github.com/marketplace/actions/stylua) GitHub Action in your CI to install and run StyLua.
This action uses the prebuilt GitHub release binaries, instead of running cargo install, for faster CI times.

### pre-commit

You can use StyLua with [pre-commit](https://pre-commit.com/).
There are 3 possible pre-commit hooks available:

- `stylua`: installs via cargo - requires the Rust toolchain
- `stylua-system`: runs a `stylua` binary available on the PATH. The binary must be pre-installed
- `stylua-github`: automatically installs the relevant prebuilt binary from GitHub Actions

Add the following to your `.pre-commit-config.yaml` file:

```yaml
- repo: https://github.com/JohnnyMorganz/StyLua
  rev: v0.19.1
  hooks:
    - id: stylua # or stylua-system / stylua-github
```

### npm

StyLua is available as a binary [published to npm](https://www.npmjs.com/package/@johnnymorganz/stylua-bin) as `@johnnymorganz/stylua-bin`.
This is a thin wrapper which installs the binary and allows it to be run through npm.

```sh
npx @johnnymorganz/stylua-bin --help
```

StyLua is also available as a WASM library at [@johnnymorganz/stylua](https://www.npmjs.com/package/@johnnymorganz/stylua).
It is usable in Node.js, or in the browser (using a bundler).

### Docker

StyLua is available on the [Docker Hub](https://hub.docker.com/r/johnnymorganz/stylua).

If you are using Docker, the easiest way to install StyLua is:

```dockerfile
COPY --from=JohnnyMorganz/StyLua:0.18.0 /stylua /usr/bin/stylua
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
aftman add johnnymorganz/stylua@0.14.2
```

- A community maintained package repository. Please note, these packages are maintained by third-parties and we do not control their packaging manifests.

[![Community Packages](https://repology.org/badge/vertical-allrepos/stylua.svg?header=Community%20Packages)](https://repology.org/project/stylua/versions)

### Other Editor Integrations

Note that these integrations require the StyLua binary to already be installed and available on your system.

- Sublime: [Sublime Text Package](https://github.com/aerobounce/Sublime-Pretty-Lua)
- Neovim: [stylua-nvim](https://github.com/ckipp01/stylua-nvim) / [stylua.nvim](https://github.com/wesleimp/stylua.nvim)

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

Note, if you are using the glob argument, it can take in multiple strings, so `--` is required to break between the glob pattern and the files to format.

By default, glob filtering (and `.styluaignore` files) are only applied for directory traversal and searching.
Files passed directly (e.g. `stylua foo.txt`) will override the glob / ignore and always be formatted.
To disable this behaviour, pass the `--respect-ignores` flag (`stylua --respect-ignores foo.txt`).

### Filtering using `.styluaignore`

You can create a `.styluaignore` file, with a format similar to `.gitignore`.
Any files matching the globs in the ignore file will be ignored by StyLua.
For example, for a `.styluaignore` file with the following contents:

```
vendor/
```

running `stylua .` will ignore the `vendor/` directory.

### `--check`: Checking files for formatting

To check whether files have been formatted (but not write directly to them), use the `--check` flag.
It will take files as input, and output a diff to stdout instead of rewriting the file contents.
If there are files which haven't been fully formatted, StyLua will exit with status code 1.

By default, we provide a custom Standard diff view, but this can be configured:

- `--output-format=unified`: output a unified diff, which can be consumed by tools like `patch` or `delta`
- `--output-format=json`: output JSON representing the changes, useful for machine-readable output

### `--verify`: Verifying formatting output

As a safety measure, the `--verify` flag can be passed to StyLua, and StyLua will verify the output of all formatting
before saving it to a file.

If enabled, the tool will re-parse the formatted output to verify if the AST is still valid (no syntax errors) and is similar to the input (possible semantic changes).

Useful when adopting StyLua in a large codebase, where it is difficult to verify all formatting is correct.
Note that this may produce false positives and negatives - we recommend manual verification as well as running tests to confirm.

### Ignoring parts of a file

To skip formatting a particular part of a file, you can add `-- stylua: ignore` before it.
This may be useful if there is a particular style you want to preseve for readability, e.g.:

```lua
-- stylua: ignore
local matrix = {
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
}
```

Formatting can also be skipped over a block of code using `-- stylua: ignore start` and `-- stylua: ignore end`:

```lua
local foo = true
-- stylua: ignore start
local   bar   =   false
local  baz      = 0
-- stylua: ignore end
local foobar = false
```

Note that ignoring cannot cross scope boundaries - once a block is exited, formatting will be re-enabled.

### Formatting Ranges

To format a specific range within a file, use `--range-start <num>` and/or `--range-end <num>`.
Both arguments are inclusive and optional - if an argument is not provided, the start/end of the file will be used respectively.

Only whole statements lying within the range will be formatted.
If part of a statement falls outside the range, the statement will be ignored.

In editors, `Format Selection` is supported.

### Requires Sorting

StyLua has built-in support for sorting require statements. We group consecutive require statements into a single "block",
and then requires are sorted only within that block. Blocks of requires do not move around the file.

We only include requires of the form `local NAME = require(EXPR)`, and sort lexicographically based on `NAME`.
(We also sort Roblox services of the form `local NAME = game:GetService(EXPR)`)

Requires sorting is off by default. To enable it, add the following to your `stylua.toml`:

```toml
[sort_requires]
enabled = true
```

## Configuration

StyLua is **opinionated**, so only a few options are provided.

### Finding the configuration

The CLI looks for `stylua.toml` or `.stylua.toml` in the directory where the tool was executed.
If not found, we search for an `.editorconfig` file, otherwise fall back to the default configuration.
This feature can be disabled using `--no-editorconfig`.
See [EditorConfig](https://editorconfig.org/) for more details.

A custom path can be provided using `--config-path <path>`.
If the path provided is not found/malformed, StyLua will exit with an error.

By default, the tool does not search further than the current directory.
Recursively searching parent directories can be enabled using `--search-parent-directories`.
This will keep searching ancestors. If not found, it will then look in `$XDG_CONFIG_HOME` / `$XDG_CONFIG_HOME/stylua`.

**Note: enabling searching outside of the current directory is NOT recommended due to possibilities of conflicting formatting:**

It is recommended to keep a `.stylua.toml` file in your project root so that other developers can make use of the same configuration.

If a project uses the default configuration of StyLua without a configuration file present, enabling external searching may cause conflicting formatting.

### Options

StyLua only offers the following options:

| Option                      | Default            | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| --------------------------- | ------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `column_width`              | `120`              | Approximate line length for printing. Used as a guide for line wrapping - this is not a hard requirement: lines may fall under or over the limit.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `line_endings`              | `Unix`             | Line endings type. Possible options: `Unix` (LF) or `Windows` (CRLF)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `indent_type`               | `Tabs`             | Indent type. Possible options: `Tabs` or `Spaces`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `indent_width`              | `4`                | Character size of single indentation. If `indent_type` is set to `Tabs`, this option is used as a heuristic to determine column width only.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `quote_style`               | `AutoPreferDouble` | Quote style for string literals. Possible options: `AutoPreferDouble`, `AutoPreferSingle`, `ForceDouble`, `ForceSingle`. `AutoPrefer` styles will prefer the specified quote style, but fall back to the alternative if it has fewer string escapes. `Force` styles always use the specified style regardless of escapes.                                                                                                                                                                                                                                                                                                                                                                                                                             |
| `call_parentheses`          | `Always`           | Whether parentheses should be applied on function calls with a single string/table argument. Possible options: `Always`, `NoSingleString`, `NoSingleTable`, `None`, `Input`. `Always` applies parentheses in all cases. `NoSingleString` omits parentheses on calls with a single string argument. Similarly, `NoSingleTable` omits parentheses on calls with a single table argument. `None` omits parentheses in both cases. Note: parentheses are still kept in situations where removal can lead to obscurity (e.g. `foo "bar".setup -> foo("bar").setup`, since the index is on the call result, not the string). `Input` removes all automation and preserves parentheses only if they were present in input code: consistency is not enforced. |
| `collapse_simple_statement` | `Never`            | Specify whether to collapse simple statements. Possible options: `Never`, `FunctionOnly`, `ConditionalOnly`, or `Always`                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

Default `stylua.toml`, note you do not need to explicitly specify each option if you want to use the defaults:

```toml
column_width = 120
line_endings = "Unix"
indent_type = "Tabs"
indent_width = 4
quote_style = "AutoPreferDouble"
call_parentheses = "Always"
collapse_simple_statement = "Never"

[sort_requires]
enabled = false
```
