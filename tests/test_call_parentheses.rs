use stylua_lib::{format_code, CallParenType, Config, OutputVerification};

fn format(paren_type: CallParenType, input: &str) -> String {
    format_code(
        input,
        Config::default().with_call_parentheses(paren_type),
        None,
        OutputVerification::None,
    )
    .unwrap()
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_always_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::Always,
            r###"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_always_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::Always,
            r###"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_no_single_string_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleString,
            r###"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_no_single_string_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleString,
            r###"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_no_single_table_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleTable,
            r###"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_no_single_table_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::NoSingleTable,
            r###"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_none_handles_string_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::None,
            r###"
local foo = require "foo"
local has_parens = require("configuration").has_parens

print [[
what ever
multi
line
string
]]
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_none_handles_table_correctly() {
    insta::assert_snapshot!(
        format(CallParenType::None,
            r###"
local opt = my_function {
    hello = true,
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"###
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
#[cfg_attr(feature = "luau", ignore)]
fn test_call_parens_has_no_affect_on_multi_arg_fn_calls_() {
    insta::assert_snapshot!(
        format(CallParenType::Always,
            r###"
local opt = my_function({
    hello = true,
}, "strarg", 5)
"###
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    }, "strarg", 5)
    "###
    );
    insta::assert_snapshot!(
        format(CallParenType::NoSingleTable,
            r###"
local opt = my_function({
    hello = true,
}, "strarg", 5)
"###
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    }, "strarg", 5)
    "###
    );
    insta::assert_snapshot!(
        format(CallParenType::None,
            r###"
local opt = my_function({
    hello = true,
}, "strarg", 5)
"###
        ),
        @r###"
    local opt = my_function({
    	hello = true,
    }, "strarg", 5)
    "###
    );
}
