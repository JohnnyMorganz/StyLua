use stylua_lib::{
    format_code, CollapseSimpleStatement, Config, OutputVerification, SortRequiresConfig,
};

fn format(input: &str) -> String {
    format_code(input, Config::default(), None, OutputVerification::None).unwrap()
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_standard() {
    insta::glob!("inputs/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg_attr(feature = "lua52", ignore)] // A test case has `goto` as an identifier, which is not allowed in Lua 5.2
fn test_full_moon_test_suite() {
    insta::glob!("inputs-full_moon/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "luau")]
fn test_luau() {
    insta::glob!("inputs-luau/*.lua", |path| {
        dbg!(path);
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "luau")]
fn test_luau_full_moon() {
    insta::glob!("inputs-luau-full_moon/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "lua52")]
fn test_lua52() {
    insta::glob!("inputs-lua52/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "lua53")]
fn test_lua53() {
    insta::glob!("inputs-lua53/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "lua54")]
fn test_lua54() {
    insta::glob!("inputs-lua54/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
fn test_ignores() {
    insta::glob!("inputs-ignore/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
fn test_collapse_single_statement() {
    insta::glob!("inputs-collapse-single-statement/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format_code(
            &contents,
            Config {
                collapse_simple_statement: CollapseSimpleStatement::Always,
                ..Config::default()
            },
            None,
            OutputVerification::None
        )
        .unwrap());
    })
}

// Collapse simple statement for goto
#[test]
#[cfg(feature = "lua52")]
fn test_collapse_single_statement_lua_52() {
    insta::assert_snapshot!(
        format_code(
            r###"
            if key == "s" then
                goto continue
            end
            "###,
            Config {
                collapse_simple_statement: CollapseSimpleStatement::Always,
                ..Config::default()
            },
            None,
            OutputVerification::None
        )
        .unwrap(),
        @r###"
    if key == "s" then goto continue end
    "###
    );
}

#[test]
fn test_sort_requires() {
    insta::glob!("inputs-sort-requires/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format_code(
            &contents,
            Config {
                sort_requires: SortRequiresConfig { enabled: true },
                ..Config::default()
            },
            None,
            OutputVerification::None
        )
        .unwrap());
    })
}

#[test]
fn test_crlf_in_multiline_comments() {
    // We need to do this outside of insta since it normalises line endings to LF
    let code = r#"
local a = "testing"
--[[
    This comment
    is multiline
    and we want to ensure the line endings
    convert to CRLF
]]
local x = 1
"#;

    let code_crlf = code.lines().collect::<Vec<_>>().join("\r\n");
    let output = format(&code_crlf);
    assert_eq!(output.find("\r\n"), None);
}

#[test]
fn test_crlf_in_multiline_strings() {
    // We need to do this outside of insta since it normalises line endings to LF
    let code = r###"
local a = [[
    This string
    is multiline
    and we want to ensure the line endings
    convert to CRLF
]]
local x = 1
"###;

    let code_crlf = code.lines().collect::<Vec<_>>().join("\r\n");
    let output = format(&code_crlf);
    assert_eq!(output.find("\r\n"), None);
}
