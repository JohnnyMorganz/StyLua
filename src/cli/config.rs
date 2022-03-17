use crate::opt::Opt;
use anyhow::{Context, Result};
use log::*;
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
fn find_config_file(mut directory: PathBuf, recursive: bool) -> Result<Option<Config>> {
    debug!("config: looking for config in {}", directory.display());
    let config_file = find_toml_file(&directory);
    match config_file {
        Some(file_path) => {
            debug!("config: found config at {}", file_path.display());
            read_config_file(&file_path).map(Some)
        }
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

/// Looks for a configuration file at either `$XDG_CONFIG_HOME`, `$XDG_CONFIG_HOME/stylua`, `$HOME/.config` or `$HOME/.config/stylua`
fn search_config_locations() -> Result<Option<Config>> {
    // Look in `$XDG_CONFIG_HOME`
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        let xdg_config_path = Path::new(&xdg_config);
        if xdg_config_path.exists() {
            debug!("config: looking in $XDG_CONFIG_HOME");

            if let Some(config) = find_config_file(xdg_config_path.to_path_buf(), false)? {
                return Ok(Some(config));
            }

            debug!("config: looking in $XDG_CONFIG_HOME/stylua");
            let xdg_config_path = xdg_config_path.join("stylua");
            if xdg_config_path.exists() {
                if let Some(config) = find_config_file(xdg_config_path, false)? {
                    return Ok(Some(config));
                }
            }
        }
    }

    // Look in `$HOME/.config`
    if let Ok(home) = std::env::var("HOME") {
        let home_config_path = Path::new(&home).join(".config");

        if home_config_path.exists() {
            debug!("config: looking in $HOME/.config");

            if let Some(config) = find_config_file(home_config_path.to_owned(), false)? {
                return Ok(Some(config));
            }

            debug!("config: looking in $HOME/.config/stylua");
            let home_config_path = home_config_path.join("stylua");
            if home_config_path.exists() {
                if let Some(config) = find_config_file(home_config_path, false)? {
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
            debug!(
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
            debug!(
                "config: starting config search from {} - recursively searching parents: {}",
                current_dir.display(),
                opt.search_parent_directories
            );
            let config = find_config_file(current_dir, opt.search_parent_directories)?;
            match config {
                Some(config) => Ok(config),
                None => {
                    debug!("config: no configuration file found");

                    // Search the configuration directory for a file, if necessary
                    if opt.search_parent_directories {
                        if let Some(config) = search_config_locations()? {
                            return Ok(config);
                        }
                    }

                    // Fallback to a default configuration
                    debug!("config: falling back to default config");
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::StructOpt;
    use stylua_lib::{CallParenType, IndentType, LineEndings, QuoteStyle};

    #[test]
    fn test_override_column_width() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--column-width", "80"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.column_width(), 80);
    }

    #[test]
    fn test_override_line_endings() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--line-endings", "Windows"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.line_endings(), LineEndings::Windows);
    }

    #[test]
    fn test_override_indent_type() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--indent-type", "Spaces"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.indent_type(), IndentType::Spaces);
    }

    #[test]
    fn test_override_indent_width() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--indent-width", "2"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.indent_width(), 2);
    }

    #[test]
    fn test_override_quote_style() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--quote-style", "ForceSingle"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.quote_style(), QuoteStyle::ForceSingle);
    }

    #[test]
    fn test_override_call_parentheses() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--call-parentheses", "None"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.call_parentheses(), CallParenType::None);
    }
}
