use stylua_lib::{format_code, Config, Range};

fn format(input: &str, range: Option<Range>) -> String {
    format_code(input, Config::default(), range).unwrap()
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_standard() {
    insta::glob!("inputs/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, None));
    })
}

#[test]
#[cfg_attr(feature = "luau", ignore)]
fn test_ranges() {
    insta::glob!("inputs-range/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        let range_contents = std::fs::read_to_string(path.with_extension("range")).unwrap();

        let mut range_info = range_contents.split("-");
        let start_range = range_info.next().unwrap().parse::<usize>().unwrap();
        let end_range = range_info.next().unwrap().parse::<usize>().unwrap();
        let range = Range::from_values(Some(start_range), Some(end_range));

        insta::assert_snapshot!(format(&contents, Some(range)));
    })
}

#[test]
#[cfg(feature = "luau")]
fn test_luau() {
    insta::glob!("inputs-luau/*.lua", |path| {
        let contents = std::fs::read_to_string(path).unwrap();
        insta::assert_snapshot!(format(&contents, None));
    })
}
