use stylua_lib::{
    format_code, BlockNewlineGaps, CollapseSimpleStatement, Config, LineEndings, LuaVersion,
    OutputVerification, SortRequiresConfig,
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

/// Comments are lifted out of trivia and re-emitted without being reformatted, so each way of
/// doing that is exercised: the source's `\r` must not survive into the output, otherwise the
/// configured line ending gets appended after it (`\r\r\n`) and formatting is not idempotent.
fn assert_crlf_normalised(input: &str) {
    assert_crlf_normalised_with_syntax(input, LuaVersion::default());
}

fn assert_crlf_normalised_with_syntax(input: &str, syntax: LuaVersion) {
    let input = input.replace('\n', "\r\n");

    for line_endings in [LineEndings::Windows, LineEndings::Unix] {
        let config = Config {
            line_endings,
            syntax,
            ..Config::default()
        };
        let once = format_code(&input, config, None, OutputVerification::None).unwrap();
        let expected_eol = match line_endings {
            LineEndings::Windows => "\r\n",
            LineEndings::Unix => "\n",
        };
        assert_eq!(
            once.replace("\r\n", "\n").replace('\n', expected_eol),
            once,
            "unexpected line endings with {:?}: {:?}",
            line_endings,
            once
        );

        let twice = format_code(&once, config, None, OutputVerification::None).unwrap();
        assert_eq!(
            once, twice,
            "formatting is not idempotent for CRLF input with {:?}",
            line_endings
        );
    }
}

#[test]
fn test_crlf_comment_in_binop_chain() {
    assert_crlf_normalised(
        "local x = \"a\"\n\t.. \"b\"\n\t-- comment\n\t.. \"c\"\nreturn x -- trailing\n",
    );
}

#[test]
fn test_crlf_comments_around_semicolon() {
    assert_crlf_normalised("local x = 1 -- a\n; -- b\nlocal y = 2\n");
}

#[test]
fn test_crlf_comments_around_assignment_equals() {
    assert_crlf_normalised("local x -- a\n= -- b\n\t1\nx -- c\n= -- d\n\t2\n");
}

#[test]
fn test_crlf_comments_around_table_field_equals() {
    assert_crlf_normalised("local t = {\n\ta -- a\n\t= -- b\n\t1,\n\t-- c\n\tb = 2, -- d\n}\n");
}

#[test]
fn test_crlf_comments_around_parentheses() {
    assert_crlf_normalised("local x = ( -- a\n\ty -- b\n) -- c\n");
}

#[test]
fn test_crlf_comments_around_function_arguments() {
    assert_crlf_normalised("call(a, -- a\n\tb, -- b\n\tc -- c\n)\n");
}

#[test]
fn test_crlf_comments_in_block_statements() {
    assert_crlf_normalised(
        "if a then -- a\n\t-- b\nelseif b then -- c\nelse -- d\nend -- e\nfor i = 1, 2 do -- f\nend\n",
    );
}

#[cfg(feature = "luau")]
#[test]
fn test_crlf_comments_in_luau_types() {
    assert_crlf_normalised("type Foo<T> = -- a\n\tT\ntype Bar = A -- b\n\t-- c\n\t| B\n");
}
