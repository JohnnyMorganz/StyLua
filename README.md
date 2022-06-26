<div align="center">
	<h1>
		StyLua<br>
		<a href="https://crates.io/crates/stylua"><img src="https://img.shields.io/crates/v/stylua.svg"></a>
    <a href="https://github.com/JohnnyMorganz/StyLua/actions/workflows/ci.yml"><img src="https://github.com/JohnnyMorganz/StyLua/actions/workflows/ci.yml/badge.svg"></a>
    <a href="https://codecov.io/gh/JohnnyMorganz/StyLua"><img src="https://codecov.io/gh/JohnnyMorganz/StyLua/branch/master/graph/badge.svg"/></a>
	</h1>
</div>

An opinionated code formatter for Lua 5.1, Lua 5.2 and [Luau](https://roblox.github.io/luau/), built using [full-moon](https://github.com/Kampfkarren/full-moon).
StyLua is inspired by the likes of [prettier](https://github.com/prettier/prettier), it parses your Lua codebase, and prints it back out from scratch,
enforcing a consistent code style.

## Installation
There are multiple ways to install StyLua:

### With Github Releases
Pre-built binaries are available on the [GitHub Releases Page](https://github.com/JohnnyMorganz/StyLua/releases).

By default, these are built with **both Luau and Lua 5.2 features enabled**, to cover all possible codebases.
If you would like to format a specific Lua version only, see [installing from crates.io](#from-cratesio).

### From Crates.io
If you have [Rust](https://www.rust-lang.org/) installed, you can install StyLua using cargo
```
cargo install stylua
```
This will compile StyLua (for Lua 5.1) and install it on your local machine.
You can pass the `--features <flag>` argument to build for Lua 5.2 (`lua52`) or Luau (`luau`):
```
cargo install stylua --features lua52
cargo install stylua --features luau
```

### GitHub Actions
You can use the [stylua-action](https://github.com/marketplace/actions/stylua) GitHub Action in your CI to install and run StyLua.
This action will use GitHub releases, rather than running cargo install, to speed up your workflow.

### pre-commit
You can use StyLua with [pre-commit](https://pre-commit.com/).
There are 3 possible pre-commit hooks available: `stylua` (which installs via Cargo, requiring the Rust toolchain to be installed),
`stylua-system` (which runs the `stylua` binary available on the PATH), and `stylua-github` (which automatically installs the relevant
StyLua prebuilt binary from GitHub Actions).
Add the following to your `.pre-commit-config.yaml` file:
```yaml
- repo: https://github.com/JohnnyMorganz/StyLua
  rev: v0.13.1
  hooks:
    - id: stylua # or stylua-system / stylua-github
```

### WASM build (npm)

StyLua is available as a wasm package, and is [published to npm](https://www.npmjs.com/package/@johnnymorganz/stylua).
It is usable in Node.js, or the in the browser (using a bundler).

### Other Installation Methods
- [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.stylua)
- [Foreman](https://github.com/Roblox/foreman) - Add the following to your `foreman.toml` file:
```toml
stylua = { source = "JohnnyMorganz/stylua", version = "0.13.1" }
```
- A community maintained package repository. Please note, these packages are maintained by third-parties and we do not control their packaging manifests.

[![Community Packages](https://repology.org/badge/vertical-allrepos/stylua.svg?header=Community%20Packages)](https://repology.org/project/stylua/versions)

### Other Editor Integrations
Note that these integrations require the StyLua binary to already be installed and available on your system.

- Sublime: [Sublime Text Package](https://github.com/aerobounce/Sublime-Pretty-Lua)
- Neovim: [stylua-nvim](https://github.com/ckipp01/stylua-nvim) / [stylua.nvim](https://github.com/wesleimp/stylua.nvim)

## Usage
Once installed, pass the files to format to the CLI:
```
stylua src/ foo.lua bar.lua
```
This command will format the `foo.lua` and `bar.lua` file, and search down the `src` directory to format any files within it.
StyLua can also read from stdin, by using `-` as the file name.

### Glob Filtering
By default, when searching through a directory, StyLua looks for all files matching the glob `**/*.lua` (or `**/*.luau` when `luau` is enabled) to format.
You can also specify an explicit glob pattern to match against when searching:
```bash
stylua --glob '**/*.luau' -- src # format all files in src matching **/*.luau
stylua -g '*.lua' -g '!*.spec.lua' -- . # format all Lua files except test files ending with `.spec.lua`
```
Note, if you are using the glob argument, it can take in multiple strings, so `--` is required to break between the glob pattern and the files to format.
If you explicitly pass a file to StyLua to format, but it doesn't match the glob, it will still be formatted (e.g. `stylua foo` for file `foo` containing Lua code)

### Filtering using `.styluaignore`
You can create a `.styluaignore` file, with a format similar to `.gitignore`. Any files matching the globs in the ignore file will be ignored by StyLua.
For example, for a `.styluaignore` file with the following contents:
```
vendor/
```
running `stylua .` will ignore the `vendor/` directory.

### `--check`: Checking files for formatting
If you want to check that files have been formatted, but not overwrite them, you can pass the `--check` argument to StyLua.
StyLua will search through files as normal, but instead of writing the formatted code back to the file, StyLua will output a diff to stdout.
If there are files which haven't been fully formatted, StyLua will exit with status code 1.

### `--verify`: Verifying formatting output
As a safety measure, the `--verify` flag can be passed to StyLua, and StyLua will verify the output of all formatting
before saving it to a file. This re-parses the output following formatting to verify that the AST is still valid and similar to the input
AST, flagging any syntax errors or possible code semantics changes. This flag may be useful when adopting StyLua on a large codebase,
where not every file can be examined for spurious formatting.

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
If you only want to format a specific range within a file, you can pass the `--range-start <num>` and/or `--range-end <num>` arguments,
and only statements within the provided range will be formatted, with the rest ignored. Both arguments are optional, and are inclusive.
If an argument is not provided, the start or end of the file will be used instead respectively.

Currently, only whole statements lying withing the range are formatted. If part of the statement is outside of the range, the statement will be ignored.

## Configuration

StyLua is **opinionated**, so only a few options are provided.

### Finding the configuration
By default, the CLI will search for a `stylua.toml` or `.stylua.toml` file in the current working directory.
If its not found, the default configuration will be used.
You can pass your own path using the `--config-path` argument, and the CLI will read the configuration present.
If the path provided is not found or the file is malformed, the CLI will exit with an error.

By default, when searching, we do not search any further than the current directory.
If you want the CLI to recursively search the parent directories for the config, the `--search-parent-directories`
flag can be used. This will keep searching, until it reaches the root path. If not found, it will look in `$XDG_CONFIG_HOME` or `$XDG_CONFIG_HOME/stylua`.
**Note: it is not recommended to use this unless necessary, as it can lead to conflicting formatting:**
If you have configuration, we recommend keeping the file in your project root so that other developers can use the same configuration, otherwise formatting styles
will be different.
Likewise, if you work on a project using StyLua, and it uses the base configuration (i.e. no config file present), you may unknowingly use
a parent/global configuration if this flag is enabled, and formatting will be unexpected.

StyLua only offers the following options:

| Option | Default | Description
| ------ | ------- | -----------
| `column_width` | `120` | The approximate line length for printing. Used as a guide to determine when to wrap lines. Note, this is not a hard requirement. Some lines may fall under or over.
| `line_endings` | `Unix` | Type of line endings to use. Possible options: `Unix` (LF) or `Windows` (CRLF)
| `indent_type` | `Tabs` | Type of indents to use. Possible options: `Tabs` or `Spaces`
| `indent_width` | `4` | The number of characters a single indent takes. If `indent_type` is set to `Tabs`, this option is used as a heuristic to determine column width only.
| `quote_style` | `AutoPreferDouble` | Types of quotes to use for string literals. Possible options: `AutoPreferDouble`, `AutoPreferSingle`, `ForceDouble`, `ForceSingle`. In `AutoPrefer` styles, we prefer the quote type specified, but fall back to the opposite if it leads to fewer escapes in the string. `Force` styles always use the style specified regardless of escapes.
| `call_parentheses` | `Always` | Specify whether to apply parentheses on function calls with a single string or table argument. Possible options: [`Always`, `NoSingleString`, `NoSingleTable`, `None`]. When `call_parentheses` is set to `Always`, StyLua applies call parentheses all the time.When it's set to `NoSingleString` it omits parentheses on function calls with single string argument. Similarly when set to `NoSingleTable` it omits parentheses on function calls with a single table argument. And when it's `None` StyLua omits parentheses on function call with single table or string argument (originally as `no_call_parentheses`). Note: parentheses are still kept in some situations if removing them will make the syntax become obscure (e.g. `foo "bar".setup -> foo("bar").setup`, as we are indexing the call result, not the string).

Default `stylua.toml`, note you do not need to explicitly specify each option if you want to use the defaults:
```toml
column_width = 120
line_endings = "Unix"
indent_type = "Tabs"
indent_width = 4
quote_style = "AutoPreferDouble"
call_parentheses = "Always"
```
