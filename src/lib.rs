use anyhow::{format_err, Result};
use full_moon::ast::owned::Owned;
use full_moon::visitors::VisitorMut;
use serde::Deserialize;

mod formatters;

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum IndentType {
    Tabs,
    Spaces,
}

impl Default for IndentType {
    fn default() -> Self {
        IndentType::Tabs
    }
}

#[derive(Debug, Copy, Clone, Deserialize)]
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

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Config {
    line_endings: LineEndings,
    indent_type: IndentType,
    indent_width: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            line_endings: LineEndings::Unix,
            indent_type: IndentType::Tabs,
            indent_width: 2
        }
    }
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
