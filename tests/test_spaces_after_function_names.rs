use stylua_lib::{format_code, Config, OutputVerification, SpaceAfterFunctionNames};

fn format(input: &str, space_after_function_names: SpaceAfterFunctionNames) -> String {
    format_code(
        input,
        Config {
            space_after_function_names,
            ..Config::default()
        },
        None,
        OutputVerification::None,
    )
    .unwrap()
}

const STARTINGCODE: &str = r###"
local foo = function() end
local function bar () end
function baz() end
a = {}
function a:b  () end
function a.c   () end
function qiz () return function () end end
foo()
bar ()
baz()
a:b  ()
a.c   ()
qiz()()
"###;

#[test]
fn test_never_space_after_function_names() {
    insta::assert_snapshot!(
        format(STARTINGCODE,
            SpaceAfterFunctionNames::Never
        ),
        @r###"
local foo = function() end
local function bar() end
function baz() end
a = {}
function a:b() end
function a.c() end
function qiz()
	return function() end
end
foo()
bar()
baz()
a:b()
a.c()
qiz()()
    "###
    );
}

#[test]
fn test_space_after_function_definitions() {
    insta::assert_snapshot!(
        format(STARTINGCODE,
            SpaceAfterFunctionNames::Definitions
        ),
        @r###"
local foo = function () end
local function bar () end
function baz () end
a = {}
function a:b () end
function a.c () end
function qiz ()
	return function () end
end
foo()
bar()
baz()
a:b()
a.c()
qiz()()
    "###
    );
}

#[test]
fn test_space_after_function_calls() {
    insta::assert_snapshot!(
        format(STARTINGCODE,
            SpaceAfterFunctionNames::Calls
        ),
        @r###"
local foo = function() end
local function bar() end
function baz() end
a = {}
function a:b() end
function a.c() end
function qiz()
	return function() end
end
foo ()
bar ()
baz ()
a:b ()
a.c ()
qiz () ()
    "###
    );
}

#[test]
fn test_always_space_after_function_names() {
    insta::assert_snapshot!(
        format(STARTINGCODE,
            SpaceAfterFunctionNames::Always
        ),
        @r###"
local foo = function () end
local function bar () end
function baz () end
a = {}
function a:b () end
function a.c () end
function qiz ()
	return function () end
end
foo ()
bar ()
baz ()
a:b ()
a.c ()
qiz () ()
    "###
    );
}
