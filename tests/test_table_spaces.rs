use stylua_lib::{format_code, BraceSpacing, Config, OutputVerification};

fn format(brace_spacing: BraceSpacing, input: &str) -> String {
    format_code(
        input,
        Config::default().with_brace_spacing(brace_spacing),
        None,
        OutputVerification::None,
    )
    .unwrap()
}

#[test]
fn test_table_oneline_with_internal_spaces() {
    insta::assert_snapshot!(
        format(BraceSpacing::Always,
            r###"
local foo = { "content" }
"###
        ),
        @r###"
    local foo = { "content" }
    "###
    );
}

#[test]
fn test_table_oneline_without_internal_spaces() {
    insta::assert_snapshot!(
        format(BraceSpacing::Never,
            r###"
local foo = { "content" }
"###
        ),
        @r###"
    local foo = {"content"}
    "###
    );
}
