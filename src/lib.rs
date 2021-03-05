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

#[derive(Debug, Copy, Clone, Deserialize)]
pub enum QuoteStyle {
    /// Use double quotes where possible, but change to single quotes if it produces less escapes
    AutoPreferDouble,
    /// Use single quotes where possible, but change to double quotes if it produces less escapes
    AutoPreferSingle,
    /// Always use double quotes in all strings
    ForceDouble,
    /// Always use single quotes in all strings
    ForceSingle,
}

impl Default for QuoteStyle {
    fn default() -> Self {
        QuoteStyle::AutoPreferDouble
    }
}

/// An optional formatting range.
/// If provided, only content within these boundaries (inclusive) will be formatted
/// Both boundaries are optional, and are given as byte offsets from the beginning of the file.
#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Range {
    start: Option<usize>,
    end: Option<usize>,
}

impl Range {
    pub fn from_values(start: Option<usize>, end: Option<usize>) -> Self {
        Self { start, end }
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    column_width: usize,
    line_endings: LineEndings,
    indent_type: IndentType,
    indent_width: usize,
    quote_style: QuoteStyle,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            column_width: 120,
            line_endings: LineEndings::Unix,
            indent_type: IndentType::Tabs,
            indent_width: 4,
            quote_style: QuoteStyle::default(),
        }
    }
}

/// Formats given Lua code
pub fn format_code(code: &str, config: Config, range: Option<Range>) -> Result<String> {
    let mut ast = match full_moon::parse(&code) {
        Ok(ast) => ast.owned(),
        Err(error) => {
            return Err(format_err!("error parsing: {}", error));
        }
    };

    let mut code_formatter = formatters::CodeFormatter::new(config, range);
    ast = code_formatter.visit_ast(ast);

    Ok(full_moon::print(&ast))
}
