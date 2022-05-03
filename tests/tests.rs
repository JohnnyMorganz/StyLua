use stylua_lib::{format_code, sort_requires::SortRequiresConfig, Config, OutputVerification};

fn format(input: &str) -> String {
    format_code(input, Config::default(), None, OutputVerification::None).unwrap()
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_standard() {
    insta::glob!("inputs/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg_attr(feature = "lua52", ignore)] // A test case has `goto` as an identifier, which is not allowed in Lua 5.2
fn test_full_moon_test_suite() {
    insta::glob!("inputs-full_moon/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "luau")]
fn test_luau() {
    insta::glob!("inputs-luau/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "luau")]
fn test_luau_full_moon() {
    insta::glob!("inputs-luau-full_moon/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
#[cfg(feature = "lua52")]
fn test_lua52() {
    insta::glob!("inputs-lua52/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents));
    })
}

#[test]
fn test_sort_requires() {
    insta::glob!("inputs-sort-requires/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format_code(
            &contents,
            Config::default().with_sort_requires(SortRequiresConfig::default().set_enabled(true)),
            None,
            OutputVerification::None
        )
        .unwrap());
    })
}
