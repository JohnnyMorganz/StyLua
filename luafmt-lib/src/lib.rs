use full_moon::ast::{owned::Owned};
use full_moon::visitors::VisitorMut;

mod formatters;

pub fn format_code(code: &'static str) -> String {
    // let ast = match full_moon::parse(&code) {
    //     Ok(ast) => ast.owned(),
    //     Err(error) => {
    //         error!("Error parsing");
    //     }
    // };

    let mut ast = full_moon::parse(&code).expect("error parse").owned();

    // We must do these in specific order, so that the latter formattings have less work to do
    ast = formatters::value_formatter::ValueFormatter::default().visit_ast(ast);
    ast = formatters::assignment_formatter::AssignmentFormatter::default().visit_ast(ast);
    ast = formatters::eof_formatter::EofFormatter::default().visit_ast(ast);

    full_moon::print(&ast)
}

#[cfg(test)]
mod tests {
    use crate::format_code;
    #[test]
    fn test_complete_format() {
        let code = "local       x     ,       y = 1   , 'foo'         \nfoo = 1";
        let output_code = "local x, y = 1, \"foo\"\nfoo = 1\n";
        assert_eq!(format_code(code), output_code);
    }
}
