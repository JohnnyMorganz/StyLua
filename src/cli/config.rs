use crate::opt::Opt;
use crate::verbose_println;
use anyhow::{Context, Result};
use directories::{ProjectDirs, UserDirs};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use stylua_lib::Config;

static CONFIG_FILE_NAME: [&str; 2] = ["stylua.toml", ".stylua.toml"];

fn read_config_file(path: &Path) -> Result<Config> {
    let contents = fs::read_to_string(path).context("Failed to read config file")?;
    let config = toml::from_str(&contents).context("Config file not in correct format")?;

    Ok(config)
}

/// Searches the directory for the configuration toml file (i.e. `stylua.toml` or `.stylua.toml`)
fn find_toml_file(directory: &Path) -> Option<PathBuf> {
    for name in &CONFIG_FILE_NAME {
        let file_path = directory.join(name);
        if file_path.exists() {
            return Some(file_path);
        }
    }

    None
}

fn find_config_file(mut directory: PathBuf, recursive: bool) -> Result<Option<Config>> {
    let config_file = find_toml_file(&directory);
    match config_file {
        Some(file_path) => read_config_file(&file_path).map(Some),
        None => {
            // Both don't exist, search up the tree if necessary
            // directory.pop() mutates the path to get its parent, and returns false if no more parent
            if recursive && directory.pop() {
                find_config_file(directory, recursive)
            } else {
                Ok(None)
            }
        }
    }
}

pub fn load_config(opt: &Opt) -> Result<Config> {
    match &opt.config_path {
        Some(config_path) => {
            verbose_println!(
                opt.verbose,
                "config: explicit config path provided at {}",
                config_path.display()
            );
            read_config_file(config_path)
        }
        None => {
            let current_dir = match &opt.stdin_filepath {
                Some(file_path) => file_path
                    .parent()
                    .context("Could not find current directory from provided stdin filepath")?
                    .to_path_buf(),
                None => env::current_dir().context("Could not find current directory")?,
            };
            verbose_println!(
                opt.verbose,
                "config: starting config search from {} - recurisvely searching parents: {}",
                current_dir.display(),
                opt.search_parent_directories
            );
            let config = find_config_file(current_dir, opt.search_parent_directories)?;
            match config {
                Some(config) => Ok(config),
                None => {
                    verbose_println!(opt.verbose, "config: no configuration file found");

                    // Search the configuration directory for a file, if necessary
                    if opt.search_parent_directories {
                        // Look in `$HOME/.config/stylua`
                        verbose_println!(opt.verbose, "config: looking in $HOME/.config/stylua");
                        if let Some(project_dirs) = ProjectDirs::from("", "", "stylua") {
                            if let Some(config) =
                                find_config_file(project_dirs.config_dir().to_path_buf(), false)?
                            {
                                return Ok(config);
                            }
                        }

                        // Look in `$HOME/.config`
                        verbose_println!(opt.verbose, "config: looking in $HOME/.config");
                        if let Some(user_dirs) = UserDirs::new() {
                            if let Some(config) =
                                find_config_file(user_dirs.home_dir().to_path_buf(), false)?
                            {
                                return Ok(config);
                            }
                        }
                    }

                    // Fallback to a default configuration
                    verbose_println!(opt.verbose, "config: falling back to default config");
                    Ok(Config::default())
                }
            }
        }
    }
}

/// Handles any overrides provided by command line options
pub fn load_overrides(config: Config, opt: &Opt) -> Config {
    let mut new_config = config;

    if let Some(column_width) = opt.format_opts.column_width {
        new_config = new_config.with_column_width(column_width);
    };
    if let Some(line_endings) = opt.format_opts.line_endings {
        new_config = new_config.with_line_endings(line_endings.into());
    };
    if let Some(indent_type) = opt.format_opts.indent_type {
        new_config = new_config.with_indent_type(indent_type.into());
    };
    if let Some(indent_width) = opt.format_opts.indent_width {
        new_config = new_config.with_indent_width(indent_width);
    };
    if let Some(quote_style) = opt.format_opts.quote_style {
        new_config = new_config.with_quote_style(quote_style.into());
    };

    new_config
}
