use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};
use stylua_lib::{IndentType, LineEndings, QuoteStyle};

lazy_static::lazy_static! {
    static ref NUM_CPUS: String = num_cpus::get().to_string();
}

#[derive(StructOpt, Debug)]
#[structopt(name = "stylua", about = "A utility to format Lua code")]
pub struct Opt {
    /// Specify path to stylua.toml configuration file
    #[structopt(long = "config-path", parse(from_os_str))]
    pub config_path: Option<PathBuf>,

    /// Specify the location of the file that is being passed into stdin.
    /// Ignored if not taking in input from stdin.
    /// This option is only used to help determine where to find the configuration file.
    #[structopt(long = "stdin-filepath", parse(from_os_str))]
    pub stdin_filepath: Option<PathBuf>,

    /// Search parent directories for stylua.toml, if not found in current directory.
    /// Ignored if config_path is provided.
    /// Keeps searching recursively up the parent directory tree, until the root directory is reached.
    /// If not found, looks in $XDG_CONFIG_HOME or $XDG_CONFIG_HOME/stylua.
    #[structopt(short, long)]
    pub search_parent_directories: bool,

    /// Runs in 'check' mode.
    /// Exits with 0 if all formatting is OK,
    /// Exits with 1 if the formatting is incorrect.
    /// Any files input will not be overwritten.
    #[structopt(short, long)]
    pub check: bool,

    /// Verify the output after formatting.
    /// Checks the generated AST with the original AST to detect if code correctness has changed.
    #[structopt(long)]
    pub verify: bool,

    /// Whether to print out verbose output
    #[structopt(short, long)]
    pub verbose: bool,

    // Whether the output should include terminal colour or not
    #[structopt(long, possible_values = &Color::variants(), case_insensitive = true, default_value = "auto")]
    pub color: Color,

    /// Any glob patterns to test against which files to check.
    /// To ignore a specific glob pattern, begin the glob pattern with `!`
    #[structopt(short, long)]
    pub glob: Option<Vec<String>>,

    /// The number of threads to use to format files in parallel. Defaults to the number of logical cores on your system.
    #[structopt(long, default_value = &NUM_CPUS)]
    pub num_threads: usize,

    /// A starting range to format files, given as a byte offset from the beginning of the file.
    /// Any content before this value will be ignored.
    #[structopt(long)]
    pub range_start: Option<usize>,

    /// An ending range to format files, given as a byte offset from the beginning of the file.
    /// Any content after this value will be ignored.
    #[structopt(long)]
    pub range_end: Option<usize>,

    /// Formatting options to apply when formatting code.
    #[structopt(flatten)]
    pub format_opts: FormatOpts,

    /// A list of files to format
    #[structopt(parse(from_os_str))]
    pub files: Vec<PathBuf>,
}

structopt::clap::arg_enum! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Color {
        Always,
        Auto,
        Never,
    }
}

#[derive(StructOpt, Debug)]
pub struct FormatOpts {
    /// The column width to use to attempt to wrap lines.
    #[structopt(long)]
    pub column_width: Option<usize>,
    /// The type of line endings to use.
    #[structopt(long, possible_values = &ArgLineEndings::variants(), case_insensitive = true, )]
    pub line_endings: Option<ArgLineEndings>,
    /// The type of indents to use.
    #[structopt(long, possible_values = &ArgIndentType::variants(), case_insensitive = true, )]
    pub indent_type: Option<ArgIndentType>,
    /// The width of a single indentation level.
    #[structopt(long)]
    pub indent_width: Option<usize>,
    /// The style of quotes to use in string literals.
    #[structopt(long, possible_values = &ArgQuoteStyle::variants(), case_insensitive = true, )]
    pub quote_style: Option<ArgQuoteStyle>,
}

// Convert [`stylua_lib::Config`] enums into clap-friendly enums
macro_rules! convert_enum {
    ($from:tt, $arg:tt, { $($enum_name:ident,)+ }) => {
        structopt::clap::arg_enum! {
            #[derive(Clone, Copy, Debug)]
            pub enum $arg {
                $(
                    $enum_name,
                )+
            }
        }

        impl From<$arg> for $from {
            fn from(other: $arg) -> $from {
                match other {
                    $(
                        $arg::$enum_name => $from::$enum_name,
                    )+
                }
            }
        }

        impl From<$from> for $arg {
            fn from(other: $from) -> $arg {
                match other {
                    $(
                        $from::$enum_name => $arg::$enum_name,
                    )+
                }
            }
        }
    };
}

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
