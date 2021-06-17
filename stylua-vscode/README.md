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

You can then use StyLua to format your code by running the `Format Document` command (In `CMD/CTRL + Shift + P`).
The `Format Selection` command is also supported, firstly highlight the code you wish to format, and select `Format Selection`.

You can also enable `editor.formatOnSave` to format your code automatically on save.

## Extension Settings

You can specify the path of the StyLua binary using the `stylua.styluaPath` setting.
By default, if this is `null`, the extension will download the binary and store it in its local storage.
