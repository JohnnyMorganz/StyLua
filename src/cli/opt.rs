use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(name = "stylua", about = "A utility to format Lua code")]
pub struct Opt {
    /// Specify path to stylua.toml configuration file
    #[structopt(long = "config-path", parse(from_os_str))]
    pub config_path: Option<PathBuf>,

    /// Search parent directories for stylua.toml, if not found in current directory.
    /// Ignored if config_path is provided.
    /// Keeps searching recursively up the parent directory tree, until the home directory is reached.
    #[structopt(short, long)]
    pub search_parent_directories: bool,

    /// Runs in 'check' mode.
    /// Exits with 0 if all formatting is OK,
    /// Exits with 1 if the formatting is incorrect.
    /// Any files input will not be overwritten.
    #[structopt(short, long)]
    pub check: bool,

    // Whether the output should include terminal colour or not
    #[structopt(long, possible_values = &Color::variants(), case_insensitive = true, default_value = "auto")]
    pub color: Color,

    /// Any glob patterns to test against which files to check.
    /// To ignore a specific glob pattern, begin the glob pattern with `!`
    #[structopt(short, long)]
    pub glob: Option<Vec<String>>,

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
