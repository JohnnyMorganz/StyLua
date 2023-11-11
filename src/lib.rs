use context::Context;
use full_moon::ast::Ast;
use serde::Deserialize;
use thiserror::Error;
#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
use wasm_bindgen::prelude::*;

#[macro_use]
mod context;
#[cfg(feature = "editorconfig")]
pub mod editorconfig;
mod formatters;
mod shape;
mod sort_requires;
mod verify_ast;

/// The type of indents to use when indenting
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "fromstr", derive(strum::EnumString))]
pub enum IndentType {
    /// Indent using tabs (`\t`)
    #[default]
    Tabs,
    /// Indent using spaces (` `)
    Spaces,
}

/// The type of line endings to use at the end of a line
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "fromstr", derive(strum::EnumString))]
pub enum LineEndings {
    // Auto,
    /// Unix Line Endings (LF) - `\n`
    #[default]
    Unix,
    /// Windows Line Endings (CRLF) - `\r\n`
    Windows,
}

/// The style of quotes to use within string literals
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "fromstr", derive(strum::EnumString))]
pub enum QuoteStyle {
    /// Use double quotes where possible, but change to single quotes if it produces less escapes
    #[default]
    AutoPreferDouble,
    /// Use single quotes where possible, but change to double quotes if it produces less escapes
    AutoPreferSingle,
    /// Always use double quotes in all strings
    ForceDouble,
    /// Always use single quotes in all strings
    ForceSingle,
}

/// When to use call parentheses
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "fromstr", derive(strum::EnumString))]
pub enum CallParenType {
    /// Use call parentheses all the time
    #[default]
    Always,
    /// Skip call parentheses when only a string argument is used.
    NoSingleString,
    /// Skip call parentheses when only a table argument is used.
    NoSingleTable,
    /// Skip call parentheses when only a table or string argument is used.
    None,
    /// Keep call parentheses based on its presence in input code.
    Input,
}

/// What mode to use if we want to collapse simple functions / guard statements
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "fromstr", derive(strum::EnumString))]
pub enum CollapseSimpleStatement {
    /// Never collapse
    #[default]
    Never,
    /// Collapse simple functions onto a single line
    FunctionOnly,
    /// Collapse simple if guards onto a single line
    ConditionalOnly,
    /// Collapse all simple statements onto a single line
    Always,
}

/// An optional formatting range.
/// If provided, only content within these boundaries (inclusive) will be formatted.
/// Both boundaries are optional, and are given as byte offsets from the beginning of the file.
#[derive(Debug, Copy, Clone, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
pub struct Range {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
impl Range {
    /// Creates a new formatting range from the given start and end point.
    /// All content within these boundaries (inclusive) will be formatted.
    pub fn from_values(start: Option<usize>, end: Option<usize>) -> Self {
        Self { start, end }
    }
}

/// Configuration for the Sort Requires codemod
#[derive(Copy, Clone, Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct SortRequiresConfig {
    /// Whether the sort requires codemod is enabled
    pub enabled: bool,
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
impl SortRequiresConfig {
    pub fn new() -> Self {
        SortRequiresConfig::default()
    }
    #[deprecated(since = "0.19.0", note = "access `.enabled` directly instead")]
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    #[deprecated(since = "0.19.0", note = "modify `.enabled` directly instead")]
    pub fn set_enabled(&self, enabled: bool) -> Self {
        Self { enabled }
    }
}

/// The configuration to use when formatting.
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Config {
    /// The approximate line length to use when printing the code.
    /// This is used as a guide to determine when to wrap lines, but note
    /// that this is not a hard upper bound.
    pub column_width: usize,
    /// The type of line endings to use.
    pub line_endings: LineEndings,
    /// The type of indents to use.
    pub indent_type: IndentType,
    /// The width of a single indentation level.
    /// If `indent_type` is set to [`IndentType::Spaces`], then this is the number of spaces to use.
    /// If `indent_type` is set to [`IndentType::Tabs`], then this is used as a heuristic to guide when to wrap lines.
    pub indent_width: usize,
    /// The style of quotes to use in string literals.
    pub quote_style: QuoteStyle,
    /// Whether to omit parentheses around function calls which take a single string literal or table.
    /// This is added for adoption reasons only, and is not recommended for new work.
    #[deprecated(note = "use `call_parentheses` instead")]
    pub no_call_parentheses: bool,
    /// When to use call parentheses.
    /// if call_parentheses is set to [`CallParenType::Always`] call parentheses is always applied.
    /// if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    /// function is called with only one string argument.
    /// if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    /// function is called with only one table argument.
    /// if call_parentheses is set to [`CallParenType::None`] call parentheses is omitted when
    /// function is called with only one table or string argument (same as no_call_parentheses).
    pub call_parentheses: CallParenType,
    /// Whether we should collapse simple structures like functions or guard statements
    /// if set to [`CollapseSimpleStatement::None`] structures are never collapsed.
    /// if set to [`CollapseSimpleStatement::FunctionOnly`] then simple functions (i.e., functions with a single laststmt) can be collapsed
    pub collapse_simple_statement: CollapseSimpleStatement,
    /// Configuration for the sort requires codemod
    pub sort_requires: SortRequiresConfig,
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
impl Config {
    /// Creates a new Config with the default values
    pub fn new() -> Self {
        Config::default()
    }

    /// Returns the current configured column width
    #[deprecated(since = "0.19.0", note = "access `.column_width` directly instead")]
    pub fn column_width(&self) -> usize {
        self.column_width
    }

    /// Returns the current configured line endings
    #[deprecated(since = "0.19.0", note = "access `.line_endings` directly instead")]
    pub fn line_endings(&self) -> LineEndings {
        self.line_endings
    }

    /// Returns the current configured indent type
    #[deprecated(since = "0.19.0", note = "access `.indent_type` directly instead")]
    pub fn indent_type(&self) -> IndentType {
        self.indent_type
    }

    /// Returns the current configured indent width
    #[deprecated(since = "0.19.0", note = "access `.indent_width` directly instead")]
    pub fn indent_width(&self) -> usize {
        self.indent_width
    }

    /// Returns the current configured quote style
    #[deprecated(since = "0.19.0", note = "access `.quote_style` directly instead")]
    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }

    /// Returns the current configured call parentheses style
    #[deprecated(since = "0.19.0", note = "access `.call_parentheses` directly instead")]
    pub fn call_parentheses(&self) -> CallParenType {
        self.call_parentheses
    }

    #[deprecated(
        since = "0.19.0",
        note = "access `.collapse_simple_statement` directly instead"
    )]
    pub fn collapse_simple_statement(&self) -> CollapseSimpleStatement {
        self.collapse_simple_statement
    }

    /// Returns the current sort requires codemod configuration
    #[deprecated(since = "0.19.0", note = "access `.sort_requires` directly instead")]
    pub fn sort_requires(&self) -> SortRequiresConfig {
        self.sort_requires
    }

    /// Returns a new config with the given column width
    #[deprecated(since = "0.19.0", note = "modify `.column_width` directly instead")]
    pub fn with_column_width(self, column_width: usize) -> Self {
        Self {
            column_width,
            ..self
        }
    }

    /// Returns a new config with the given line endings
    #[deprecated(since = "0.19.0", note = "modify `.line_endings` directly instead")]
    pub fn with_line_endings(self, line_endings: LineEndings) -> Self {
        Self {
            line_endings,
            ..self
        }
    }

    /// Returns a new config with the given indent type
    #[deprecated(since = "0.19.0", note = "modify `.indent_type` directly instead")]
    pub fn with_indent_type(self, indent_type: IndentType) -> Self {
        Self {
            indent_type,
            ..self
        }
    }

    /// Returns a new config with the given indent width
    #[deprecated(since = "0.19.0", note = "modify `.indent_width` directly instead")]
    pub fn with_indent_width(self, indent_width: usize) -> Self {
        Self {
            indent_width,
            ..self
        }
    }

    /// Returns a new config with the given quote style
    #[deprecated(since = "0.19.0", note = "modify `.quote_style` directly instead")]
    pub fn with_quote_style(self, quote_style: QuoteStyle) -> Self {
        Self {
            quote_style,
            ..self
        }
    }

    /// Returns a new config with the given value for `no_call_parentheses`
    #[deprecated(note = "use `call_parentheses")]
    pub fn with_no_call_parentheses(self, no_call_parentheses: bool) -> Self {
        #[allow(deprecated)]
        Self {
            no_call_parentheses,
            ..self
        }
    }

    /// Returns a new config with the given call parentheses type
    #[deprecated(since = "0.19.0", note = "modify `.call_parentheses` directly instead")]
    pub fn with_call_parentheses(self, call_parentheses: CallParenType) -> Self {
        Self {
            call_parentheses,
            ..self
        }
    }

    #[deprecated(
        since = "0.19.0",
        note = "modify `.collapse_simple_statement` directly instead"
    )]
    pub fn with_collapse_simple_statement(
        self,
        collapse_simple_statement: CollapseSimpleStatement,
    ) -> Self {
        Self {
            collapse_simple_statement,
            ..self
        }
    }

    /// Returns a new config with the given sort requires configuration
    #[deprecated(since = "0.19.0", note = "modify `.sort_requires` directly instead")]
    pub fn with_sort_requires(self, sort_requires: SortRequiresConfig) -> Self {
        Self {
            sort_requires,
            ..self
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        #[allow(deprecated)]
        Self {
            column_width: 120,
            line_endings: LineEndings::default(),
            indent_type: IndentType::default(),
            indent_width: 4,
            quote_style: QuoteStyle::default(),
            no_call_parentheses: false,
            call_parentheses: CallParenType::default(),
            collapse_simple_statement: CollapseSimpleStatement::default(),
            sort_requires: SortRequiresConfig::default(),
        }
    }
}

/// The type of verification to perform to validate that the output AST is still correct.
#[derive(Debug, Copy, Clone, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
pub enum OutputVerification {
    /// Reparse the generated output to detect any changes to code correctness.
    Full,
    /// Perform no verification of the output.
    None,
}

/// A formatting error
#[derive(Clone, Debug, Error)]
pub enum Error {
    /// The input AST has a parsing error.
    #[error("error parsing: {0}")]
    ParseError(full_moon::Error),
    /// The output AST after formatting generated a parse error. This is a definite error.
    #[error("INTERNAL ERROR: Output AST generated a syntax error. Please report this at https://github.com/johnnymorganz/stylua/issues\n{0}")]
    VerificationAstError(full_moon::Error),
    /// The output AST after formatting differs from the input AST.
    #[error("INTERNAL WARNING: Output AST may be different to input AST. Code correctness may have changed. Please examine the formatting diff and report any issues at https://github.com/johnnymorganz/stylua/issues")]
    VerificationAstDifference,
}

/// Formats given [`Ast`]
#[allow(clippy::result_large_err)]
pub fn format_ast(
    input_ast: Ast,
    config: Config,
    range: Option<Range>,
    verify_output: OutputVerification,
) -> Result<Ast, Error> {
    // Clone the input AST only if we are verifying, to later use for checking
    let input_ast_for_verification = if let OutputVerification::Full = verify_output {
        Some(input_ast.to_owned())
    } else {
        None
    };

    let ctx = Context::new(config, range);

    // Perform require sorting beforehand if necessary
    let input_ast = match config.sort_requires.enabled {
        true => sort_requires::sort_requires(&ctx, input_ast),
        false => input_ast,
    };

    let code_formatter = formatters::CodeFormatter::new(ctx);
    let ast = code_formatter.format(input_ast);

    // If we are verifying, reparse the output then check it matches the original input
    if let Some(input_ast) = input_ast_for_verification {
        let output = full_moon::print(&ast);
        let reparsed_output = match full_moon::parse(&output) {
            Ok(ast) => ast,
            Err(error) => {
                return Err(Error::VerificationAstError(error));
            }
        };

        let mut ast_verifier = verify_ast::AstVerifier::new();
        if !ast_verifier.compare(input_ast, reparsed_output) {
            return Err(Error::VerificationAstDifference);
        }
    }

    Ok(ast)
}

/// Formats given Lua code
#[allow(clippy::result_large_err)]
pub fn format_code(
    code: &str,
    config: Config,
    range: Option<Range>,
    verify_output: OutputVerification,
) -> Result<String, Error> {
    let input_ast = match full_moon::parse(code) {
        Ok(ast) => ast,
        Err(error) => {
            return Err(Error::ParseError(error));
        }
    };

    let ast = format_ast(input_ast, config, range, verify_output)?;
    let output = full_moon::print(&ast);

    Ok(output)
}

#[cfg(all(target_arch = "wasm32", feature = "wasm-bindgen"))]
#[wasm_bindgen(js_name = formatCode)]
pub fn format_code_wasm(
    code: &str,
    config: Config,
    range: Option<Range>,
    verify_output: OutputVerification,
) -> Result<String, String> {
    format_code(code, config, range, verify_output).map_err(|err| err.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_point() {
        let output = format_code(
            "local   x   =    1",
            Config::default(),
            None,
            OutputVerification::None,
        )
        .unwrap();
        assert_eq!(output, "local x = 1\n");
    }

    #[test]
    fn test_invalid_input() {
        let output = format_code(
            "local   x   = ",
            Config::default(),
            None,
            OutputVerification::None,
        );
        assert!(matches!(output, Err(Error::ParseError(_))))
    }

    #[test]
    fn test_with_ast_verification() {
        let output = format_code(
            "local   x   =    1",
            Config::default(),
            None,
            OutputVerification::Full,
        )
        .unwrap();
        assert_eq!(output, "local x = 1\n");
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_column_width() {
        let new_config = Config::new().with_column_width(80);
        assert_eq!(new_config.column_width(), 80);
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_line_endings() {
        let new_config = Config::new().with_line_endings(LineEndings::Windows);
        assert_eq!(new_config.line_endings(), LineEndings::Windows);
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_indent_type() {
        let new_config = Config::new().with_indent_type(IndentType::Spaces);
        assert_eq!(new_config.indent_type(), IndentType::Spaces);
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_indent_width() {
        let new_config = Config::new().with_indent_width(2);
        assert_eq!(new_config.indent_width(), 2);
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_quote_style() {
        let new_config = Config::new().with_quote_style(QuoteStyle::ForceDouble);
        assert_eq!(new_config.quote_style(), QuoteStyle::ForceDouble);
    }

    #[test]
    #[allow(deprecated)]
    fn test_config_call_parentheses() {
        let new_config = Config::new().with_call_parentheses(CallParenType::None);
        assert_eq!(new_config.call_parentheses(), CallParenType::None);
    }
}
