# StyLua VSCode Extension

StyLua is an opinionated code formatter for Lua 5.1 and [Luau](https://roblox.github.io/luau/), built using [full-moon](https://github.com/Kampfkarren/full-moon).
StyLua is inspired by the likes of [prettier](https://github.com/prettier/prettier), it parses your Lua codebase, and prints it back out from scratch,
enforcing a consistent code style.

## Information

For more information, see the main [repository](https://github.com/JohnnyMorganz/StyLua)

## Usage

Set StyLua as your formatter when prompted, or add the following configuration to your `settings.json` file:

```json
"[lua]": {
    "editor.defaultFormatter": "JohnnyMorganz.stylua"
},
```

If you are working with Luau code, you may need to also configure under the `luau` namespace:

```json
"[luau]": {
    "editor.defaultFormatter": "JohnnyMorganz.stylua"
},
```

You can then use StyLua to format your code by running the `Format Document` command (In `CMD/CTRL + Shift + P`).
The `Format Selection` command is also supported, firstly highlight the code you wish to format, and select `Format Selection`.

You can also enable `editor.formatOnSave` to format your code automatically on save.

## Finding a StyLua binary

You can specify the path of the StyLua binary using the `stylua.styluaPath` setting.

If no configuration is specified, then we lookup the version through the following steps:

1. If `stylua.searchBinaryInPATH` is enabled, then lookup a "stylua" binary on the PATH.

- If found, run `stylua --version` to ensure it executes appropriately

2. If binary not found on PATH, or does not execute, then fall back to a bundled version.

The bundled version of StyLua is downloaded from GitHub releases, and stored in your local storage.

By default, the extension downloads the latest version. To configure the version to use, set `stylua.targetReleaseVersion`.
Or alternatively, use the `Stylua: Select Version` command.

When a new StyLua version is available, you will be notified on the VSCode status bar. You can disable these notifications
via `stylua.disableVersionCheck`.
