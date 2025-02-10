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

/// The Lua syntax version to use
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "fromstr", derive(strum::EnumString))]
pub enum LuaVersion {
    /// Parse all syntax versions at the same time. This allows most general usage.
    /// For overlapping syntaxes (e.g., Lua5.2 label syntax and Luau type assertions), select a
    /// specific syntax version
    #[default]
    All,
    /// Parse Lua 5.1 code
    Lua51,
    /// Parse Lua 5.2 code
    #[cfg(feature = "lua52")]
    Lua52,
    /// Parse Lua 5.3 code
    #[cfg(feature = "lua53")]
    Lua53,
    /// Parse Lua 5.4 code
    #[cfg(feature = "lua54")]
    Lua54,
    /// Parse Luau code
    #[cfg(feature = "luau")]
    Luau,
    /// Parse LuaJIT code
    #[cfg(feature = "luajit")]
    LuaJIT,
    /// Parse CFX Lua code
    #[cfg(feature = "cfxlua")]
    CFXLua,
}

impl From<LuaVersion> for full_moon::LuaVersion {
    fn from(val: LuaVersion) -> Self {
        match val {
            LuaVersion::All => full_moon::LuaVersion::new(),
            LuaVersion::Lua51 => full_moon::LuaVersion::lua51(),
            #[cfg(feature = "lua52")]
            LuaVersion::Lua52 => full_moon::LuaVersion::lua52(),
            #[cfg(feature = "lua53")]
            LuaVersion::Lua53 => full_moon::LuaVersion::lua53(),
            #[cfg(feature = "lua54")]
            LuaVersion::Lua54 => full_moon::LuaVersion::lua54(),
            #[cfg(feature = "luau")]
            LuaVersion::Luau => full_moon::LuaVersion::luau(),
            #[cfg(feature = "luajit")]
            LuaVersion::LuaJIT => full_moon::LuaVersion::luajit(),
            #[cfg(feature = "cfxlua")]
            LuaVersion::CFXLua => full_moon::LuaVersion::cfxlua(),
        }
    }
}

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
    #[cfg(not(all(target_arch = "wasm32", feature = "wasm-bindgen")))]
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    #[deprecated(since = "0.19.0", note = "modify `.enabled` directly instead")]
    pub fn set_enabled(&self, enabled: bool) -> Self {
        Self { enabled }
    }
}

/// When to use spaces after function names
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Deserialize)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "fromstr", derive(strum::EnumString))]
pub enum SpaceAfterFunctionNames {
    /// Never use spaces after function names.
    #[default]
    Never,
    /// Use spaces after function names only for function definitions.
    Definitions,
    /// Use spaces after function names only for function calls.
    Calls,
    /// Use spaces after function names in definitions and calls.
    Always,
}

/// The configuration to use when formatting.
#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
pub struct Config {
    /// The type of Lua syntax to parse.
    pub syntax: LuaVersion,
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
    /// Whether we should include a space between the function name and arguments.
    /// * if space_after_function_names is set to [`SpaceAfterFunctionNames::Never`] a space is never used.
    /// * if space_after_function_names is set to [`SpaceAfterFunctionNames::Definitions`] a space is used only for definitions.
    /// * if space_after_function_names is set to [`SpaceAfterFunctionNames::Calls`] a space is used only for calls.
    /// * if space_after_function_names is set to [`SpaceAfterFunctionNames::Always`] a space is used for both definitions and calls.
    pub space_after_function_names: SpaceAfterFunctionNames,
}

#[cfg_attr(all(target_arch = "wasm32", feature = "wasm-bindgen"), wasm_bindgen)]
impl Config {
    /// Creates a new Config with the default values
    pub fn new() -> Self {
        Config::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        #[allow(deprecated)]
        Self {
            syntax: LuaVersion::default(),
            column_width: 120,
            line_endings: LineEndings::default(),
            indent_type: IndentType::default(),
            indent_width: 4,
            quote_style: QuoteStyle::default(),
            no_call_parentheses: false,
            call_parentheses: CallParenType::default(),
            collapse_simple_statement: CollapseSimpleStatement::default(),
            sort_requires: SortRequiresConfig::default(),
            space_after_function_names: SpaceAfterFunctionNames::default(),
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

fn print_full_moon_error(error: &full_moon::Error) -> String {
    match error {
        full_moon::Error::AstError(ast_error) => format!(
            "unexpected token `{}` ({}:{} to {}:{}), {}",
            ast_error.token(),
            ast_error.range().0.line(),
            ast_error.range().0.character(),
            ast_error.range().1.line(),
            ast_error.range().1.character(),
            ast_error.error_message()
        ),
        full_moon::Error::TokenizerError(tokenizer_error) => tokenizer_error.to_string(),
    }
}

fn print_full_moon_errors(errors: &[full_moon::Error]) -> String {
    if errors.len() == 1 {
        print_full_moon_error(errors.first().unwrap())
    } else {
        errors
            .iter()
            .map(|err| "\n - ".to_string() + &print_full_moon_error(err))
            .collect::<String>()
    }
}

/// A formatting error
#[derive(Clone, Debug, Error)]
pub enum Error {
    /// The input AST has a parsing error.
    #[error("error parsing: {}", print_full_moon_errors(.0))]
    ParseError(Vec<full_moon::Error>),
    /// The output AST after formatting generated a parse error. This is a definite error.
    #[error("INTERNAL ERROR: Output AST generated a syntax error. Please report this at https://github.com/johnnymorganz/stylua/issues: {}", print_full_moon_errors(.0))]
    VerificationAstError(Vec<full_moon::Error>),
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
        let output = ast.to_string();
        let reparsed_output =
            match full_moon::parse_fallible(&output, config.syntax.into()).into_result() {
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
    let input_ast = match full_moon::parse_fallible(code, config.syntax.into()).into_result() {
        Ok(ast) => ast,
        Err(error) => {
            return Err(Error::ParseError(error));
        }
    };

    let ast = format_ast(input_ast, config, range, verify_output)?;
    let output = ast.to_string();

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
}
