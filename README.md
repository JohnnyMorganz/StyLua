# StyLua

An opinionated code formatter for Lua 5.1 and [Luau](https://roblox.github.io/luau/), built using [full-moon](https://github.com/Kampfkarren/full-moon).
StyLua is inspired by the likes of [prettier](https://github.com/prettier/prettier), it parses your Lua codebase, and prints it back out from scratch,
enforcing a consistent code style.

## Installation
There are multiple ways to install StyLua:

### With Github Releases
Pre-built binaries are available on the [GitHub Releases Page](https://github.com/JohnnyMorganz/StyLua/releases).
Please note, currently by default, StyLua is built with Luau features enabled. If you would just like to format Lua 5.1 code, please see installing from crates.io

### With [Foreman](https://github.com/Roblox/foreman)
StyLua can be installed using foreman. Add the following to your `foreman.toml` file:
```
stylua = { source = "JohnnyMorganz/stylua", version = "0.2.1" }
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

## Configuration

StyLua is **opinionated**, so there are as little configuration options as possible.
The CLI will search for a `stylua.toml` file in the current working directory to read the configuration.
Alternatively, you can pass your own path using the `--config-path` argument.

StyLua only offers the following options:

### `line_endings`

The type of line endings to use, supports either `Unix` (LF) or `Windows` (CRLF) options.
Defaults to `Unix`.

```
line_endings = "Unix"
```

### `indent_type`

The type of indents to use, supports either `Tabs` or `Spaces`.
Defaults to `Tabs`.

```
indent_type = "Tabs"
```

### `indent_width`

The width of spaces a single indent level should be. This option is ignored if the `indent_type` is set to `Tabs`.
Defaults to `4`.

```
indent_width = 2
```
