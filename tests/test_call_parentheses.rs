use stylua_lib::{format_code, CallParenType, Config, OutputVerification};

fn format(call_parentheses: CallParenType, input: &str) -> String {
    format_code(
        input,
        Config {
            call_parentheses,
            ..Config::default()
        },
        None,
        OutputVerification::None,
    )
    .unwrap()
}

#[test]
fn test_call_parens_always_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::Always,
            r#"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"#
        ),
        @r###"
    local foo = require("foo")
    local has_parens = require("configuration").has_parens

    print([[
    what ever
    multi
    line
    string
    ]])
    "###
    );
}

#[test]
fn test_call_parens_always_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::Always,
            r#"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"#
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    })

    local still_got_em = my_function({
    	config = true,
    	value = "yup",
    }):method()
    "###
    );
}

#[test]
fn test_call_parens_no_single_string_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleString,
            r#"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"#
        ),
        @r###"
    local foo = require "foo"
    local has_parens = require("configuration").has_parens

    print [[
    what ever
    multi
    line
    string
    ]]
    "###
    );
}

#[test]
fn test_call_parens_no_single_string_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleString,
            r#"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"#
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    })

    local still_got_em = my_function({
    	config = true,
    	value = "yup",
    }):method()
    "###
    );
}

#[test]
fn test_call_parens_no_single_table_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleTable,
            r#"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"#
        ),
        @r###"
    local foo = require("foo")
    local has_parens = require("configuration").has_parens

    print([[
    what ever
    multi
    line
    string
    ]])
    "###
    );
}

#[test]
fn test_call_parens_no_single_table_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleTable,
            r#"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"#
        ),
        @r###"
    local opt = my_function {
    	hello = true,
    }

    local still_got_em = my_function({
    	config = true,
    	value = "yup",
    }):method()
    "###
    );
}

#[test]
fn test_call_parens_none_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::None,
            r#"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"#
        ),
        @r###"
    local foo = require "foo"
    local has_parens = require("configuration").has_parens

    print [[
    what ever
    multi
    line
    string
    ]]
    "###
    );
}

#[test]
fn test_call_parens_none_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::None,
            r#"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"#
        ),
        @r###"
    local opt = my_function {
    	hello = true,
    }

    local still_got_em = my_function({
    	config = true,
    	value = "yup",
    }):method()
    "###
    );
}

#[test]
fn test_call_parens_has_no_affect_on_multi_arg_fn_calls_() {
    insta::assert_snapshot!(
        format(CallParenType::Always,
            r#"
local opt = my_function({
    hello = true,
}, "strarg", 5)
"#
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    }, "strarg", 5)
    "###
    );
    insta::assert_snapshot!(
        format(CallParenType::NoSingleTable,
            r#"
local opt = my_function({
    hello = true,
}, "strarg", 5)
"#
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    }, "strarg", 5)
    "###
    );
    insta::assert_snapshot!(
        format(CallParenType::None,
            r#"
local opt = my_function({
    hello = true,
}, "strarg", 5)
"#
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    }, "strarg", 5)
    "###
    );
}

#[test]
fn test_call_parens_comments() {
    insta::assert_snapshot!(
        format(CallParenType::None,
            r#"
foo("hello") -- comment
"#
        ),
        @r###"foo "hello" -- comment
    "###
    );
}

#[test]
fn test_call_parens_semicolons() {
    insta::assert_snapshot!(
        format(CallParenType::None,
            r#"
foo"hello"; -- comment
foo{ x = y }; -- comment
"#
        ),
        @r###"
    foo "hello" -- comment
    foo { x = y } -- comment
    "###
    );
}

#[test]
fn test_call_parens_input() {
    insta::assert_snapshot!(
        format(CallParenType::Input,
            r#"
require("path")
local x = New "TextLabel" {
    x = game:FindFirstChild("Testing")
}
"#
        ),
        @r###"
    require("path")
    local x = New "TextLabel" {
    	x = game:FindFirstChild("Testing"),
    }
    "###
    );
}
