#[cfg(all(feature = "luau", any(feature = "lua52", feature = "lua53")))]
use stylua_lib::{format_code, Config, OutputVerification};

#[test]
#[cfg(all(feature = "luau", feature = "lua53"))]
fn test_hint_syntax_luau_for_generic_parameter_list() {
    let code = r#"
        export type Foo = A<B<C>>
    "#;
    let result = format_code(code, Config::default(), None, OutputVerification::None);
    let error = result.unwrap_err().to_string();
    assert!(error.ends_with("hint: this looks like a conflict with Lua 5.3 and Luau generics syntax, add `syntax = \"Luau\"` to stylua.toml to resolve"));
}

#[test]
#[cfg(all(feature = "luau", feature = "lua52"))]
fn test_hint_syntax_lua52_for_labels() {
    let code = r#"
        do
            local x = 1
            ::continue::
        end
    "#;
    let result = format_code(code, Config::default(), None, OutputVerification::None);
    let error = result.unwrap_err().to_string();
    assert!(error.ends_with("hint: this looks like a conflict with Luau and Lua 5.2 label syntax, add `syntax = \"Lua52\"` to stylua.toml to resolve"));
}
