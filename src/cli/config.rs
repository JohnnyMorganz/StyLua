use crate::opt::Opt;
use crate::verbose_println;
use anyhow::{Context, Result};
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

/// Looks for a configuration file in the directory provided (and its parent's recursively, if specified)
fn find_config_file(
    mut directory: PathBuf,
    recursive: bool,
    verbose: bool,
) -> Result<Option<Config>> {
    verbose_println!(
        verbose,
        "config: looking for config in {}",
        directory.display()
    );
    let config_file = find_toml_file(&directory);
    match config_file {
        Some(file_path) => {
            verbose_println!(verbose, "config: found config at {}", file_path.display());
            read_config_file(&file_path).map(Some)
        }
        None => {
            // Both don't exist, search up the tree if necessary
            // directory.pop() mutates the path to get its parent, and returns false if no more parent
            if recursive && directory.pop() {
                find_config_file(directory, recursive, verbose)
            } else {
                Ok(None)
            }
        }
    }
}

/// Looks for a configuration file at either `$XDG_CONFIG_HOME`, `$XDG_CONFIG_HOME/stylua`, `$HOME/.config` or `$HOME/.config/stylua`
fn search_config_locations(verbose: bool) -> Result<Option<Config>> {
    // Look in `$XDG_CONFIG_HOME`
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        let xdg_config_path = Path::new(&xdg_config);
        if xdg_config_path.exists() {
            verbose_println!(verbose, "config: looking in $XDG_CONFIG_HOME");

            if let Some(config) = find_config_file(xdg_config_path.to_path_buf(), false, verbose)? {
                return Ok(Some(config));
            }

            verbose_println!(verbose, "config: looking in $XDG_CONFIG_HOME/stylua");
            let xdg_config_path = xdg_config_path.join("stylua");
            if xdg_config_path.exists() {
                if let Some(config) = find_config_file(xdg_config_path, false, verbose)? {
                    return Ok(Some(config));
                }
            }
        }
    }

    // Look in `$HOME/.config`
    if let Ok(home) = std::env::var("HOME") {
        let home_config_path = Path::new(&home).join(".config");

        if home_config_path.exists() {
            verbose_println!(verbose, "config: looking in $HOME/.config");

            if let Some(config) = find_config_file(home_config_path.to_owned(), false, verbose)? {
                return Ok(Some(config));
            }

            verbose_println!(verbose, "config: looking in $HOME/.config/stylua");
            let home_config_path = home_config_path.join("stylua");
            if home_config_path.exists() {
                if let Some(config) = find_config_file(home_config_path, false, verbose)? {
                    return Ok(Some(config));
                }
            }
        }
    }

    Ok(None)
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
                "config: starting config search from {} - recursively searching parents: {}",
                current_dir.display(),
                opt.search_parent_directories
            );
            let config = find_config_file(current_dir, opt.search_parent_directories, opt.verbose)?;
            match config {
                Some(config) => Ok(config),
                None => {
                    verbose_println!(opt.verbose, "config: no configuration file found");

                    // Search the configuration directory for a file, if necessary
                    if opt.search_parent_directories {
                        if let Some(config) = search_config_locations(opt.verbose)? {
                            return Ok(config);
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
    if let Some(call_parentheses) = opt.format_opts.call_parentheses {
        new_config = new_config.with_call_parentheses(call_parentheses.into());
    };

    new_config
}
