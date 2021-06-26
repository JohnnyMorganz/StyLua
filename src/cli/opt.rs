use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

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
