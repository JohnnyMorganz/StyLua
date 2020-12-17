use anyhow::{format_err, Result};
use full_moon::ast::owned::Owned;
use full_moon::visitors::VisitorMut;

mod formatters;

/// Formats given Lua code
pub fn format_code(code: &str) -> Result<String> {
    let mut ast = match full_moon::parse(&code) {
        Ok(ast) => ast.owned(),
        Err(error) => {
            return Err(format_err!("error parsing: {}", error));
        }
    };

    ast = formatters::CodeFormatter::default().visit_ast(ast);

    Ok(full_moon::print(&ast))
}
