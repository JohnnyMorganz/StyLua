use clap::{ArgEnum, StructOpt};
use std::path::PathBuf;
use stylua_lib::{
    CallParenType, CollapseSimpleStatement, IndentType, LineEndings, LuaVersion, QuoteStyle,
    SpaceAfterFunctionNames,
};

lazy_static::lazy_static! {
    static ref NUM_CPUS: String = num_cpus::get().to_string();
}

#[derive(StructOpt, Clone, Debug)]
#[structopt(name = "stylua", about = "A utility to format Lua code", version)]
pub struct Opt {
    /// Specify path to stylua.toml configuration file.
    ///
    /// If not provided, defaults to looking in the current directory for a configuration file.
    #[structopt(long = "config-path", short = 'f', parse(from_os_str))]
    pub config_path: Option<PathBuf>,

    /// Specify the location of the file that is being passed into stdin.
    /// Ignored if not taking in input from stdin.
    ///
    /// This option is only used to help determine where to find the configuration file.
    #[structopt(long = "stdin-filepath", parse(from_os_str))]
    pub stdin_filepath: Option<PathBuf>,

    /// Search parent directories for stylua.toml, if not found in current directory.
    /// Ignored if config_path is provided.
    ///
    /// Keeps searching recursively up the parent directory tree, until the root directory is reached.
    /// If not found, looks in $XDG_CONFIG_HOME or $XDG_CONFIG_HOME/stylua.
    #[structopt(short, long)]
    pub search_parent_directories: bool,

    /// Runs in 'check' mode.
    ///
    /// Compares a diff between all input files to determine if they are formatted.
    /// Exits with 0 if all formatting is OK,
    /// Exits with 1 if any formatting is incorrect, outputting file diffs.
    /// Any files input will not be overwritten.
    #[structopt(short, long)]
    pub check: bool,

    /// Configures the diff output when using 'check' mode.
    ///
    /// This option is ignored if 'check' is not enabled.
    #[structopt(long, arg_enum, ignore_case = true, default_value_t = OutputFormat::Standard)]
    pub output_format: OutputFormat,

    /// Verifies the output correctness after formatting.
    ///
    /// Checks the generated AST with the original AST to detect if code correctness has changed.
    #[structopt(long)]
    pub verify: bool,

    /// Enables verbose output
    #[structopt(short, long)]
    pub verbose: bool,

    /// Use colored output.
    #[structopt(long, ignore_case = true, default_value_t = Color::Auto, arg_enum)]
    pub color: Color,

    /// Glob patterns to test against which files to check.
    ///
    /// To ignore a specific glob pattern, begin the glob pattern with `!`
    #[structopt(short, long)]
    pub glob: Option<Vec<String>>,

    /// The number of threads to use to format files in parallel.
    ///
    /// Defaults to the number of logical cores on your system.
    #[structopt(long, default_value = &NUM_CPUS)]
    pub num_threads: usize,

    /// A starting range to format files, given as a byte offset from the beginning of the file.
    ///
    /// Any content before this value will be ignored.
    #[structopt(long)]
    pub range_start: Option<usize>,

    /// An ending range to format files, given as a byte offset from the beginning of the file.
    ///
    /// Any content after this value will be ignored.
    #[structopt(long)]
    pub range_end: Option<usize>,

    /// Formatting options to apply when formatting code.
    #[structopt(flatten, next_help_heading = "FORMATTING OPTIONS")]
    pub format_opts: FormatOpts,

    /// A list of files to format
    #[structopt(parse(from_os_str))]
    pub files: Vec<PathBuf>,

    /// Whether to traverse hidden files/directories.
    #[structopt(short, long)]
    pub allow_hidden: bool,

    /// Disables the EditorConfig feature.
    ///
    /// Has no effect if a stylua.toml configuration file is found.
    #[cfg(feature = "editorconfig")]
    #[structopt(long)]
    pub no_editorconfig: bool,

    /// Respect .styluaignore and glob matching for file paths provided directly to the tool
    #[structopt(long)]
    pub respect_ignores: bool,
}

#[derive(ArgEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "PascalCase")]
pub enum Color {
    /// Always use colour
    Always,
    /// Checks the terminal features to determine whether to apply colour
    Auto,
    /// Never use colour
    Never,
}

impl Color {
    pub fn should_use_color(&self) -> bool {
        match self {
            Color::Always => true,
            Color::Never => false,
            Color::Auto => {
                let terminal = console::Term::stdout();
                let features = terminal.features();
                features.is_attended() && features.colors_supported()
            }
        }
    }

    pub fn should_use_color_stderr(&self) -> bool {
        match self {
            Color::Always => true,
            Color::Never => false,
            Color::Auto => {
                let terminal = console::Term::stderr();
                let features = terminal.features();
                features.is_attended() && features.colors_supported()
            }
        }
    }
}

#[derive(ArgEnum, Clone, Copy, Debug)]
#[clap(rename_all = "PascalCase")]
pub enum OutputFormat {
    /// Outputs using the standard inbuilt pretty-diff design
    Standard,
    /// Outputs using unified diff formatting
    Unified,
    /// Outputs in json
    Json,
    /// Outputs a human-friendly summary
    Summary,
}

#[derive(StructOpt, Clone, Copy, Debug)]
pub struct FormatOpts {
    /// The type of Lua syntax to parse
    #[structopt(long, arg_enum, ignore_case = true)]
    pub syntax: Option<ArgLuaVersion>,
    /// The column width to use to attempt to wrap lines.
    #[structopt(long)]
    pub column_width: Option<usize>,
    /// The type of line endings to use.
    #[structopt(long, arg_enum, ignore_case = true)]
    pub line_endings: Option<ArgLineEndings>,
    /// The type of indents to use.
    #[structopt(long, arg_enum, ignore_case = true)]
    pub indent_type: Option<ArgIndentType>,
    /// The width of a single indentation level.
    #[structopt(long)]
    pub indent_width: Option<usize>,
    /// The style of quotes to use in string literals.
    #[structopt(long, arg_enum, ignore_case = true)]
    pub quote_style: Option<ArgQuoteStyle>,
    /// Specify whether to apply parentheses on function calls with single string or table arg.
    #[structopt(long, arg_enum, ignore_case = true)]
    pub call_parentheses: Option<ArgCallParenType>,
    /// Specify whether to collapse simple statements.
    #[structopt(long, arg_enum, ignore_case = true)]
    pub collapse_simple_statement: Option<ArgCollapseSimpleStatement>,
    /// Enable requires sorting
    #[structopt(long)]
    pub sort_requires: bool,
    #[structopt(long, arg_enum, ignore_case = true)]
    pub space_after_function_names: Option<ArgSpaceAfterFunctionNames>,
}

// Convert [`stylua_lib::Config`] enums into clap-friendly enums
macro_rules! convert_enum {
    ($from:tt, $arg:tt, { $($(#[$inner:meta])* $enum_name:ident,)+ }) => {
        #[derive(ArgEnum, Clone, Copy, Debug)]
        #[clap(rename_all = "PascalCase")]
        pub enum $arg {
            $(
                $(#[$inner])*
                $enum_name,
            )+
        }

        impl From<$arg> for $from {
            fn from(other: $arg) -> $from {
                match other {
                    $(
                        $(#[$inner])*
                        $arg::$enum_name => $from::$enum_name,
                    )+
                }
            }
        }

        impl From<$from> for $arg {
            fn from(other: $from) -> $arg {
                match other {
                    $(
                        $(#[$inner])*
                        $from::$enum_name => $arg::$enum_name,
                    )+
                }
            }
        }
    };
}

convert_enum!(LuaVersion, ArgLuaVersion, {
    All,
    Lua51,
    #[cfg(feature = "lua52")] Lua52,
    #[cfg(feature = "lua53")] Lua53,
    #[cfg(feature = "lua54")] Lua54,
    #[cfg(feature = "luau")] Luau,
    #[cfg(feature = "luajit")] LuaJIT,
    #[cfg(feature = "cfxlua")] CfxLua,
});

convert_enum!(LineEndings, ArgLineEndings, {
    Unix,
    Windows,
});

convert_enum!(IndentType, ArgIndentType, {
    Tabs,
    Spaces,
});

convert_enum!(QuoteStyle, ArgQuoteStyle, {
    AutoPreferDouble,
    AutoPreferSingle,
    ForceDouble,
    ForceSingle,
});

convert_enum!(CallParenType, ArgCallParenType, {
    Always,
    NoSingleString,
    NoSingleTable,
    None,
    Input,
});

convert_enum!(CollapseSimpleStatement, ArgCollapseSimpleStatement, {
    Never,
    FunctionOnly,
    ConditionalOnly,
    Always,
});

convert_enum!(SpaceAfterFunctionNames, ArgSpaceAfterFunctionNames, {
    Never,
    Definitions,
    Calls,
    Always,
});

#[cfg(test)]
mod tests {
    use super::Opt;
    use clap::IntoApp;

    #[test]
    fn verify_opt() {
        Opt::command().debug_assert()
    }
}
