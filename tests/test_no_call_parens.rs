use stylua_lib::{format_code, Config};

fn format(input: &str) -> String {
    format_code(
        input,
        Config::default().with_no_call_parentheses(true),
        None,
    )
    .unwrap()
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_no_parens_string() {
    insta::assert_snapshot!(
        format(
            r###"
foo"string"
"###
        ),
        @r###"foo "string"
    "###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_omit_parens_string() {
    insta::assert_snapshot!(
        format(
            r###"
foo("string")
"###
        ),
        @r###"foo "string"
    "###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_no_parens_brackets_string() {
    insta::assert_snapshot!(
        format(
            r###"
foo [[
    string
]]
"###
        ),
        @r###"
    foo [[
        string
    ]]
    "###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_omit_parens_brackets_string() {
    insta::assert_snapshot!(
        format(
            r###"
foo([[
    string
]])
"###
        ),
        @r###"
    foo [[
        string
    ]]
    "###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_no_parens_singleline_table() {
    insta::assert_snapshot!(
        format(
            r###"
foo{bar=true}
"###
        ),
        @"foo { bar = true }
    "
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_no_parens_multiline_table() {
    insta::assert_snapshot!(
        format(
            r###"
foo{
    bar=true
}
"###
        ),
        @r###"
    foo {
    	bar = true,
    }
    "###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_keep_parens_binop_string() {
    insta::assert_snapshot!(
        format(
            r###"
foo("foo" .. "bar")
"###
        ),
        @r###"foo("foo" .. "bar")
    "###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_no_parens_method_chain_1() {
    insta::assert_snapshot!(
        format(
            r###"
foo("foo"):andThen()
"###
        ),
        @r###"foo("foo"):andThen()
"###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_no_parens_method_chain_2() {
    insta::assert_snapshot!(
        format(
            r###"
Job:new({
    foo = "bar"
}):sync()
"###
        ),
        @r###"
    Job
    	:new({
    		foo = "bar",
    	})
    	:sync()
    "###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_no_parens_large_example() {
    insta::assert_snapshot!(
        format(
            r###"
local foo = require "foo"
local has_parens = require("configuration").has_parens

local opt = my_function {
    hello = true
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"###
        ),
        @r###"
    local foo = require "foo"
    local has_parens = require("configuration").has_parens

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
