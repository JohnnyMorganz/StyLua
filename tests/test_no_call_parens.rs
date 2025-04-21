use stylua_lib::{format_code, Config, OutputVerification};

fn format(input: &str) -> String {
    format_code(
        input,
        #[allow(deprecated)]
        Config {
            no_call_parentheses: true,
            ..Config::default()
        },
        None,
        OutputVerification::None,
    )
    .unwrap()
}

#[test]
fn test_no_parens_string() {
    insta::assert_snapshot!(
        format(
            r#"
foo"string"
"#
        ),
        @r###"foo "string"
    "###
    );
}

#[test]
fn test_omit_parens_string() {
    insta::assert_snapshot!(
        format(
            r#"
foo("string")
"#
        ),
        @r###"foo "string"
    "###
    );
}

#[test]
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
fn test_keep_parens_binop_string() {
    insta::assert_snapshot!(
        format(
            r#"
foo("foo" .. "bar")
"#
        ),
        @r###"foo("foo" .. "bar")
    "###
    );
}

#[test]
fn test_no_parens_method_chain_1() {
    insta::assert_snapshot!(
        format(
            r#"
foo("foo"):andThen()
"#
        ),
        @r###"foo("foo"):andThen()
    "###
    );
}

#[test]
fn test_no_parens_method_chain_2() {
    insta::assert_snapshot!(
        format(
            r#"
Job:new({
    foo = "bar"
}):sync()
"#
        ),
        @r###"
    Job:new({
    	foo = "bar",
    }):sync()
    "###
    );
}

#[test]
fn test_no_parens_large_example() {
    insta::assert_snapshot!(
        format(
            r#"
local foo = require "foo"
local has_parens = require("configuration").has_parens

local opt = my_function {
    hello = true
}

local still_got_em = my_function({
    config = true,
    value = "yup",
}):method()
"#
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

#[test]
fn test_keep_parens_for_leading_comment() {
    insta::assert_snapshot!(
        format(
            r#"
foo( -- test
"string")

foo(
  -- test
"string")

foo( -- test
    {})

foo(
  -- test
        {})
"#
        ),
        @r#"
    foo( -- test
    	"string"
    )

    foo(
    	-- test
    	"string"
    )

    foo( -- test
    	{}
    )

    foo(
    	-- test
    	{}
    )
    "#
    );
}

#[test]
fn test_keep_parens_for_trailing_comment() {
    insta::assert_snapshot!(
        format(
            r#"
foo("string" -- test
)

foo("string"
    -- test
)

foo({} -- test
)

foo({}
    -- test
)
"#
        ),
        @r#"
    foo "string" -- test

    foo(
    	"string"
    	-- test
    )

    foo {} -- test

    foo(
    	{}
    	-- test
    )
    "#
    );
}

#[test]
fn test_keep_parentheses_large_example() {
    insta::assert_snapshot!(
        format(
            r#"
wk.add(
  { { key, "zv" .. key, "Same, but open folds" } }
  -- { noremap = true, mode = { 'n', key:find '^<' and 'v' or 'x' } }
)

wk.add(
  -- { noremap = true, mode = { 'n', key:find '^<' and 'v' or 'x' } }
  { { key, "zv" .. key, "Same, but open folds" } }
)
"#
        ),
        @r#"
    wk.add(
    	{ { key, "zv" .. key, "Same, but open folds" } }
    	-- { noremap = true, mode = { 'n', key:find '^<' and 'v' or 'x' } }
    )

    wk.add(
    	-- { noremap = true, mode = { 'n', key:find '^<' and 'v' or 'x' } }
    	{ { key, "zv" .. key, "Same, but open folds" } }
    )
    "#
    );
}
