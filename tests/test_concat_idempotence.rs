use stylua_lib::{format_code, Config, OutputVerification};

#[test]
fn test_concat_multiline_string_chain_idempotent() {
    let input = include_str!("inputs/concat-multiline-string-chain.lua");
    let format =
        |code: &str| format_code(code, Config::default(), None, OutputVerification::None).unwrap();
    let first = format(input);
    assert_eq!(format(&first), first);
}
