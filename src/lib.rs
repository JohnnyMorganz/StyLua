use serde::Deserialize;

#[macro_use]
mod context;
mod formatters;
mod shape;
mod verify_ast;

/// The type of indents to use when indenting
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum IndentType {
    /// Indent using tabs (`\t`)
    Tabs,
    /// Indent using spaces (` `)
    Spaces,
}

impl Default for IndentType {
    fn default() -> Self {
        IndentType::Tabs
    }
}

/// The type of line endings to use at the end of a line
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum LineEndings {
    // Auto,
    /// Unix Line Endings (LF) - `\n`
    Unix,
    /// Windows Line Endings (CRLF) - `\r\n`
    Windows,
}

impl Default for LineEndings {
    fn default() -> Self {
        LineEndings::Unix
    }
}

/// The style of quotes to use within string literals
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

/// When to use call parentheses
#[derive(Debug, Copy, Clone, PartialEq, Deserialize)]
pub enum CallParenType {
    /// Use call parentheses all the time
    Always,
    /// Skip call parentheses when only a string argument is used.
    NoSingleString,
    /// Skip call parentheses when only a table argument is used.
    NoSingleTable,
    /// Skip call parentheses when only a table or string argument is used.
    None,
}

impl Default for CallParenType {
    fn default() -> Self {
        CallParenType::Always
    }
}

/// An optional formatting range.
/// If provided, only content within these boundaries (inclusive) will be formatted.
/// Both boundaries are optional, and are given as byte offsets from the beginning of the file.
#[derive(Debug, Copy, Clone, Deserialize)]
pub struct Range {
    start: Option<usize>,
    end: Option<usize>,
}

impl Range {
    /// Creates a new formatting range from the given start and end point.
    /// All content within these boundaries (inclusive) will be formatted.
    pub fn from_values(start: Option<usize>, end: Option<usize>) -> Self {
        Self { start, end }
    }
}

/// The configuration to use when formatting.
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    /// The approximate line length to use when printing the code.
    /// This is used as a guide to determine when to wrap lines, but note
    /// that this is not a hard upper bound.
    column_width: usize,
    /// The type of line endings to use.
    line_endings: LineEndings,
    /// The type of indents to use.
    indent_type: IndentType,
    /// The width of a single indentation level.
    /// If `indent_type` is set to [`IndentType::Spaces`], then this is the number of spaces to use.
    /// If `indent_type` is set to [`IndentType::Tabs`], then this is used as a heuristic to guide when to wrap lines.
    indent_width: usize,
    /// The style of quotes to use in string literals.
    quote_style: QuoteStyle,
    /// Whether to omit parentheses around function calls which take a single string literal or table.
    /// This is added for adoption reasons only, and is not recommended for new work.
    no_call_parentheses: bool,
    /// When to use call parentheses.
    /// if call_parentheses is set to [`CallParenType::Always`] call parentheses is always applied.
    /// if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    /// function is called with only one string argument.
    /// if call_parentheses is set to [`CallParenType::NoSingleTable`] call parentheses is omitted when
    /// function is called with only one table argument.
    /// if call_parentheses is set to [`CallParenType::None`] call parentheses is omitted when
    /// function is called with only one table or string argument (same as no_call_parentheses).
    call_parentheses: CallParenType,
}

impl Config {
    /// Creates a new Config with the default values
    pub fn new() -> Self {
        Config::default()
    }

    /// Returns a new config with the given column width
    pub fn with_column_width(self, column_width: usize) -> Self {
        Self {
            column_width,
            ..self
        }
    }

    /// Returns a new config with the given line endings
    pub fn with_line_endings(self, line_endings: LineEndings) -> Self {
        Self {
            line_endings,
            ..self
        }
    }

    /// Returns a new config with the given indent type
    pub fn with_indent_type(self, indent_type: IndentType) -> Self {
        Self {
            indent_type,
            ..self
        }
    }

    /// Returns a new config with the given indent width
    pub fn with_indent_width(self, indent_width: usize) -> Self {
        Self {
            indent_width,
            ..self
        }
    }

    /// Returns a new config with the given quote style
    pub fn with_quote_style(self, quote_style: QuoteStyle) -> Self {
        Self {
            quote_style,
            ..self
        }
    }

    /// Returns a new config with the given value for [`no_call_parentheses`]
    pub fn with_no_call_parentheses(self, no_call_parentheses: bool) -> Self {
        Self {
            no_call_parentheses,
            ..self
        }
    }
    /// Returns a new config with the given call parentheses type
    pub fn with_call_parentheses(self, call_parentheses: CallParenType) -> Self {
        Self {
            call_parentheses,
            ..self
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            column_width: 120,
            line_endings: LineEndings::Unix,
            indent_type: IndentType::Tabs,
            indent_width: 4,
            quote_style: QuoteStyle::default(),
            no_call_parentheses: false,
            call_parentheses: CallParenType::default(),
        }
    }
}

/// The type of verification to perform to validate that the output AST is still correct.
#[derive(Debug, Copy, Clone, Deserialize)]
pub enum OutputVerification {
    /// Reparse the generated output to detect any changes to code correctness.
    Full,
    /// Perform no verification of the output.
    None,
}

#[derive(Clone, Debug)]
pub enum Error {
    /// The input AST has a parsing error.
    ParseError(full_moon::Error),
    /// The output AST after formatting generated a parse error. This is a definite error.
    VerificationAstError(full_moon::Error),
    /// The output AST after formatting differs from the input AST.
    VerificationAstDifference,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::ParseError(error) => match error {
                full_moon::Error::AstError(error) => write!(formatter, "error parsing: {}", error),
                full_moon::Error::TokenizerError(error) => write!(formatter, "error parsing: {}", error),
            },
            Error::VerificationAstError(error) => write!(formatter, "INTERNAL ERROR: Output AST generated a syntax error. Please report this at https://github.com/johnnymorganz/stylua/issues\n{}", error),
            Error::VerificationAstDifference => write!(formatter, "INTERNAL WARNING: Output AST may be different to input AST. Code correctness may have changed. Please examine the formatting diff and report any issues at https://github.com/johnnymorganz/stylua/issues"),
        }
    }
}

impl std::error::Error for Error {}

/// Formats given Lua code
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

    // Clone the input AST only if we are verifying, to later use for checking
    let input_ast_for_verification = if let OutputVerification::Full = verify_output {
        Some(input_ast.to_owned())
    } else {
        None
    };

    let code_formatter = formatters::CodeFormatter::new(config, range);
    let ast = code_formatter.format(input_ast);
    let output = full_moon::print(&ast);

    // If we are verifying, reparse the output then check it matches the original input
    if let Some(input_ast) = input_ast_for_verification {
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

    Ok(output)
}
