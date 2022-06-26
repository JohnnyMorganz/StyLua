use stylua_lib::{format_code, CollapseMode, Config, OutputVerification};

fn format(collapse_mode: CollapseMode, input: &str) -> String {
    format_code(
        input,
        Config::default().with_collapse_mode(collapse_mode),
        None,
        OutputVerification::None,
    )
    .unwrap()
}

#[test]
fn test_collapse_simple_functions() {
    insta::assert_snapshot!(
        format(CollapseMode::Functions,
            r###"
function foo()
    return bar
end
"###
        ),
        @r###"
    function foo() return bar end
    "###
    );
}
