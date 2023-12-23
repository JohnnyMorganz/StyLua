use stylua_lib::{format_code, Config, OutputVerification, SpaceAfterFunctions};

fn format(input: &str, space_after_functions: SpaceAfterFunctions) -> String {
    format_code(
        input,
        Config {
            space_after_functions,
            ..Config::default()
        },
        None,
        OutputVerification::None,
    )
    .unwrap()
}

const STARTINGCODE: &str = r#"
local foo = function() end
local function bar () end
function baz() end
foo()
bar ()
baz()
"#;

#[test]
fn test_never_space_after_functions() {
    insta::assert_snapshot!(
        format(STARTINGCODE,
            SpaceAfterFunctions::Never
        ),
        @r###"
local foo = function() end
local function bar() end
function baz() end
foo()
bar()
baz()
    "###
    );
}

#[test]
fn test_space_after_function_definitions() {
    insta::assert_snapshot!(
        format(STARTINGCODE,
            SpaceAfterFunctions::Definitions
        ),
        @r###"
local foo = function () end
local function bar () end
function baz () end
foo()
bar()
baz()
    "###
    );
}

#[test]
fn test_always_space_after_functions() {
    insta::assert_snapshot!(
        format(STARTINGCODE,
            SpaceAfterFunctions::Always
        ),
        @r###"
local foo = function () end
local function bar () end
function baz () end
foo ()
bar ()
baz ()
    "###
    );
}
