# StyLua

An opinionated code formatter for Lua 5.1, Lua 5.2 and [Luau](https://roblox.github.io/luau/), built using [full-moon](https://github.com/Kampfkarren/full-moon).
StyLua is inspired by the likes of [prettier](https://github.com/prettier/prettier), it parses your Lua codebase, and prints it back out from scratch,
enforcing a consistent code style.

## Installation
There are multiple ways to install StyLua:

### With Github Releases
Pre-built binaries are available on the [GitHub Releases Page](https://github.com/JohnnyMorganz/StyLua/releases).

Please note, currently by default, **StyLua is built with Luau features enabled**. If you would just like to format Lua 5.1 code,
or would like to format Lua 5.2 code, please see [installing from crates.io](#from-cratesio)

### From Crates.io
If you have [Rust](https://www.rust-lang.org/) installed, you can install StyLua using cargo
```
cargo install stylua
```
This will compile StyLua (for Lua 5.1) and install it on your local machine.
If you would like Luau features, pass the `--features luau` argument.
```
cargo install stylua --features luau
```
Similarly, for Lua 5.2 syntax, pass the `--features lua52` argument.
```
cargo install stylua --features lua52
```

### With [Foreman](https://github.com/Roblox/foreman)
StyLua can be installed using foreman. Add the following to your `foreman.toml` file:
```toml
stylua = { source = "JohnnyMorganz/stylua", version = "0.8.1" }
```

### Using the VSCode Extension

You can use the [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.stylua), which will automatically download StyLua for you.
Set StyLua as your formatter when prompted, or add the following configuration to your `settings.json` file:

```json
"[lua]": {
    "editor.defaultFormatter": "JohnnyMorganz.stylua"
},
```

and StyLua will then be used to format your code. It is recommended to also enable `editor.formatOnSave`.

### GitHub Actions
You can use the [stylua-action](https://github.com/marketplace/actions/stylua) GitHub Action in your CI to install and run StyLua efficiently.
This action will use GitHub releases, rather than running cargo install, to speed up your workflow.

## Usage
Once installed, using StyLua is quick and simple, just pass the files to format to the CLI.
```
stylua src/ foo.lua bar.lua
```
This command will format the `foo.lua` and `bar.lua` file, and search down the `src` directory to format any files within it.

StyLua can also read from stdin, by using `-` as the file name.

### Glob Filtering
When searching through a directory, a glob pattern can be used to specify which specific types of files to format:
```
stylua --glob **/*.lua -- src
```
Multiple glob patterns can be used to match specific files, and not others. For example:
```
stylua -g *.lua -g !*.spec.lua -- .
```
will format all Lua files, but ignore any `.spec.lua` test files.
Note, if you are using the glob argument, it can take in multiple strings, so a `--` is required to break between the glob pattern and the files to format.
The glob defaults to `**/*.lua`.
If you explicitly pass a file to StyLua to format, but it doesn't match the glob, it will still be formatted (e.g. `stylua foo` for file `foo` containing Lua code)

### Filtering using `.styluaignore`
You can also create a `.styluaignore` file, with a similar format to a `.gitignore` file. Any files matched will be ignored by StyLua.
For example, for a `.styluaignore` file with the following contents:
```
vendor/
```
running `stylua .` will ignore the `vendor/` directory.

### Checking files for formatting
If you want to check that files have been formatted, but not overwrite them, you can pass the `--check` argument to StyLua.
StyLua will search through files as normal, but instead of writing the formatted code back to the file, StyLua will output a diff to stdout.
If there are files which haven't been fully formatted, StyLua will exit with status code 1.

### Formatting Ranges
If you only want to format a specific range within a file, you can pass the `--range-start <num>` and/or `--range-end <num>` arguments,
and only statements within the provided range will be formatted, with the rest ignored. Both arguments are optional, and are inclusive.
If an argument is not provided, the start or end of the file will be used instead respectively.

Currently, only whole statements lying withing the range are formatted. If part of the statement is outside of the range, the statement will be ignored.

There is also support for the formatting selected ranges in the [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=JohnnyMorganz.stylua).

### Ignoring parts of a file
If there is a specific statement within your file which you wish to skip formatting on, you can precede it with `-- stylua: ignore`,
and it will be skipped over during formatting. This may be useful when there is a specific formatting style you wish to preserve for
a statement. For example:
```lua
-- stylua: ignore
local matrix = {
    { 0, 0, 0 },
    { 0, 0, 0 },
    { 0, 0, 0 },
}
```

## Configuration

StyLua is **opinionated**, so there are as little configuration options as possible.

### Finding the configuration
By default, the CLI will search for a `stylua.toml` or `.stylua.toml` file in the current working directory.
If its not found, the default configuration will be used.
You can pass your own path using the `--config-path` argument, and the CLI will read the configuration present.
If the path provided is not found or the file is malformed, the CLI will exit with an error.

By default, when searching, we do not search any further than the current directory.
If you want the CLI to recursively search the parent directories for the config, the `--search-parent-directories`
flag can be used. This will keep searching, until it reaches the root path. If not found, it will look in `$XDG_CONFIG_HOME` or `$XDG_CONFIG_HOME/stylua`.
**Note: this is a separate flag for a reason, it is not recommended to use this unless necessary.**
If you have configuration, we recommend keeping the file in your project root so that other developers can use the same configuration, otherwise formatting styles
will be different. Likewise, if you work on a project using StyLua, and it uses the base configuration (i.e. no config file present), you may unwantingly use
a parent/global configuration if this flag is enabled, and formatting will be different.

StyLua only offers the following options:

### `column_width`

The approximate line length for printing. This is used as a guide to determine when to wrap lines, but note this is
not a hard upper bound.
Defaults to `120`.

```toml
column_width = 120
```

### `line_endings`

The type of line endings to use, supports either `Unix` (LF) or `Windows` (CRLF) options.
Defaults to `Unix`.

```toml
line_endings = "Unix"
```

### `indent_type`

The type of indents to use, supports either `Tabs` or `Spaces`.
Defaults to `Tabs`.

```toml
indent_type = "Tabs"
```

### `indent_width`

The width of spaces a single indent level should be. This option is used for heuristics only to determine column width if the `indent_type` is set to `Tabs`.
Defaults to `4`.

```toml
indent_width = 2
```

### `quote_style`

The types of quotes to use for string literals, supports either `AutoPreferDouble`, `AutoPreferSingle`, `ForceDouble` or `ForceSingle`.
For the auto styles, we will prefer the quote type specified, but fall back to the opposite if it means there are fewer escapes in the string. For the
force styles, we will always use the quote type specified.
Defaults to `AutoPreferDouble`.

```toml
quote_style = "AutoPreferDouble"
```

### `no_call_parentheses`

When enabled, parentheses are removed around function arguments where a single string literal/table is passed.
Note: if the function call is followed by an index or a method call, parentheses are added/kept. This is because
the syntax becomes ambiguous.
```lua
require("foobar") -> require "foobar"
something({ foo = bar }) -> something { foo = bar }

-- keep/add parentheses due to obscurity
-- it looks like its indexing the string, but its actually indexing the return from the function call
getsomething "foobar".setup -> getsomething("foobar").setup
setup { yes = true }:run() -> setup({ yes = true }):run()
```
This option was added for adoption purposes.
Defaults to `false`

```toml
no_call_parentheses = false
```