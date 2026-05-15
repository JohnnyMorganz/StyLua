use stylua_lib::{
    format_code, BlockNewlineGaps, CollapseSimpleStatement, Config, LuaVersion, OutputVerification,
    SortRequiresConfig,
};

fn format(input: &str, syntax: LuaVersion) -> String {
    let config = Config {
        syntax,
        ..Config::default()
    };
    format_code(input, config, None, OutputVerification::None).unwrap()
}

#[test]
fn test_standard() {
    insta::glob!("inputs/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Lua51));
    })
}

#[test]
fn test_full_moon_test_suite() {
    insta::glob!("inputs-full_moon/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Lua51));
    })
}

#[test]
#[cfg(feature = "luau")]
fn test_luau() {
    insta::glob!("inputs-luau/*.lua", |path| {
        dbg!(path);
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Luau));
    })
}

#[test]
#[cfg(feature = "luau")]
fn test_luau_full_moon() {
    insta::glob!("inputs-luau-full_moon/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Luau));
    })
}

#[test]
#[cfg(feature = "lua52")]
fn test_lua52() {
    insta::glob!("inputs-lua52/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Lua52));
    })
}

#[test]
#[cfg(feature = "lua53")]
fn test_lua53() {
    insta::glob!("inputs-lua53/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Lua53));
    })
}

#[test]
#[cfg(feature = "lua54")]
fn test_lua54() {
    insta::glob!("inputs-lua54/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Lua54));
    })
}

#[test]
#[cfg(feature = "luajit")]
fn test_luajit() {
    insta::glob!("inputs-luajit/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::LuaJIT));
    })
}

#[test]
#[cfg(feature = "cfxlua")]
fn test_cfxlua() {
    insta::glob!("inputs-cfxlua/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::CfxLua));
    })
}

#[test]
fn test_ignores() {
    insta::glob!("inputs-ignore/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, LuaVersion::Lua51));
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

#[test]
fn test_collapse_simple_statement_input_preserves_singleline() {
    let config = Config {
        collapse_simple_statement: CollapseSimpleStatement::Input,
        ..Config::default()
    };

    insta::assert_snapshot!(
        format_code(
            r#"function foo() return  bar end
if x then return  y end
"#,
            config,
            None,
            OutputVerification::None
        )
        .unwrap(),
        @r###"
    function foo() return bar end
    if x then return y end
    "###
    );
}

#[test]
fn test_collapse_simple_statement_input_preserves_multiline() {
    let config = Config {
        collapse_simple_statement: CollapseSimpleStatement::Input,
        ..Config::default()
    };

    insta::assert_snapshot!(
        format_code(
            r#"function foo()
    return  bar
end
if x then
    return  y
end
"#,
            config,
            None,
            OutputVerification::None
        )
        .unwrap(),
        @r###"
    function foo()
    	return bar
    end
    if x then
    	return y
    end
    "###
    );
}

#[test]
fn test_collapse_simple_statement_input_preserves_empty_function_shape() {
    let config = Config {
        collapse_simple_statement: CollapseSimpleStatement::Input,
        ..Config::default()
    };

    insta::assert_snapshot!(
        format_code(
            r#"function singleline() end
function multiline()
end
"#,
            config,
            None,
            OutputVerification::None
        )
        .unwrap(),
        @r###"
    function singleline() end
    function multiline()
    end
    "###
    );
}

#[test]
fn test_collapse_simple_statement_input_preserves_singleline_over_width() {
    let config = Config {
        collapse_simple_statement: CollapseSimpleStatement::Input,
        column_width: 40,
        ..Config::default()
    };

    insta::assert_snapshot!(
        format_code(
            r#"function fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo() return  bar end
if fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo then return  bar end
"#,
            config,
            None,
            OutputVerification::None
        )
        .unwrap(),
        @r###"
    function fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo() return bar end
    if fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo then return bar end
    "###
    );
}

#[test]
fn test_collapse_simple_statement_input_preserves_function_argument_hug() {
    let config = Config {
        collapse_simple_statement: CollapseSimpleStatement::Input,
        column_width: 20,
        ..Config::default()
    };

    insta::assert_snapshot!(
        format_code(
            r#"task.spawn(function()

end)
"#,
            config,
            None,
            OutputVerification::None
        )
        .unwrap(),
        @r###"
    task.spawn(function()
    end)
    "###
    );
}

#[test]
fn test_collapse_simple_statement_input_preserves_multi_argument_function_hug() {
    let config = Config {
        collapse_simple_statement: CollapseSimpleStatement::Input,
        column_width: 20,
        ..Config::default()
    };

    insta::assert_snapshot!(
        format_code(
            r#"x.SomeMethod("something", function()

end)
"#,
            config,
            None,
            OutputVerification::None
        )
        .unwrap(),
        @r###"
    x.SomeMethod("something", function()
    end)
    "###
    );
}

#[test]
fn test_preserve_block_newline_gaps() {
    insta::glob!("inputs-preserve-block-newline-gaps/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format_code(
            &contents,
            Config {
                block_newline_gaps: BlockNewlineGaps::Preserve,
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
                syntax: LuaVersion::Lua52,
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
    let output = format(&code_crlf, LuaVersion::Lua51);
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
    let output = format(&code_crlf, LuaVersion::Lua51);
    assert_eq!(output.find("\r\n"), None);
}
