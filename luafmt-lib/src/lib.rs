use anyhow::{format_err, Result};
use full_moon::ast::owned::Owned;
use full_moon::visitors::VisitorMut;

mod formatters;

#[derive(Debug)]
pub enum IndentType {
    Tabs,
    Spaces,
}

impl Default for IndentType {
    fn default() -> Self {
        IndentType::Tabs
    }
}

#[derive(Debug)]
pub enum LineEndings {
    // Auto,
    Unix,
    Windows,
}

impl Default for LineEndings {
    fn default() -> Self {
        LineEndings::Unix
    }
}

#[derive(Default, Debug)]
pub struct Config {
    line_endings: LineEndings,
    indent_type: IndentType,
}

/// Formats given Lua code
pub fn format_code(code: &str, config: Config) -> Result<String> {
    let mut ast = match full_moon::parse(&code) {
        Ok(ast) => ast.owned(),
        Err(error) => {
            return Err(format_err!("error parsing: {}", error));
        }
    };

    let mut code_formatter = formatters::CodeFormatter::new(config);
    ast = code_formatter.visit_ast(ast);

    Ok(full_moon::print(&ast))
}
