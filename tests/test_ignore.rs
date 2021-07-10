use stylua_lib::{format_code, Config, OutputVerification};

fn format(input: &str) -> String {
    format_code(input, Config::default(), None, OutputVerification::None).unwrap()
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_singleline_ignore() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore
local bar   =     baz
            "###
        ),
        @r###""###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_singleline_ignore_2() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore
local bar   =     baz
local bar   =     baz
            "###
        ),
        @r###""###
    );
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_multiline_block_ignore() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore start
local bar   =     baz
-- stylua: ignore end
local bar   =     baz
"###
        ),
    @r###"
    "###);
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_multiline_block_ignore_2() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore start
local bar   =     baz
local bar   =     baz
-- stylua: ignore end
local bar   =     baz
"###
        ),
    @r###"
    "###);
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_multiline_block_ignore_no_ending() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
-- stylua: ignore start
local bar   =     baz
local bar   =     baz
local bar   =     baz
"###
        ),
    @r###"
    "###);
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_multiline_block_ignore_no_starting() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
local bar   =     baz
local bar   =     baz
-- stylua: ignore end
local bar   =     baz
"###
        ),
    @r###"
    "###);
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_multiline_block_ignore_block_scope() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
do
    -- stylua: ignore start
    local bar   =     baz
    -- stylua: ignore end
    local bar   =     baz
end
local bar   =     baz
"###
        ),
    @r###"
    "###);
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_multiline_block_ignore_block_scope_no_ending() {
    insta::assert_snapshot!(
        format(
            r###"
local foo     =      bar
do
    -- stylua: ignore start
    local bar   =     baz
    local bar   =     baz
end
local bar   =     baz
"###
        ),
    @r###"
    "###);
}
