use crate::opt::Opt;
use anyhow::{Context, Result};
use log::*;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use stylua_lib::Config;
use stylua_lib::SortRequiresConfig;

#[cfg(feature = "editorconfig")]
use stylua_lib::editorconfig;

static CONFIG_FILE_NAME: [&str; 2] = ["stylua.toml", ".stylua.toml"];

fn read_config_file(path: &Path) -> Result<Config> {
    let contents = fs::read_to_string(path).context("Failed to read config file")?;
    let config = toml::from_str(&contents).context("Config file not in correct format")?;

    Ok(config)
}

fn read_and_apply_overrides(path: &Path, opt: &Opt) -> Result<Config> {
    read_config_file(path).map(|config| load_overrides(config, opt))
}

pub struct ConfigResolver<'a> {
    config_cache: HashMap<PathBuf, Option<Config>>,
    forced_configuration: Option<Config>,
    current_directory: PathBuf,
    default_configuration: Config,
    opt: &'a Opt,
}

impl ConfigResolver<'_> {
    pub fn new(opt: &Opt) -> Result<ConfigResolver> {
        let forced_configuration = opt
            .config_path
            .as_ref()
            .map(|config_path| {
                debug!(
                    "config: explicit config path provided at {}",
                    config_path.display()
                );
                read_and_apply_overrides(config_path, opt)
            })
            .transpose()?;

        Ok(ConfigResolver {
            config_cache: HashMap::new(),
            forced_configuration,
            current_directory: env::current_dir().context("Could not find current directory")?,
            default_configuration: load_overrides(Config::default(), opt),
            opt,
        })
    }

    /// Returns the root used when searching for configuration
    /// If `--search-parent-directories`, then there is no root, and we keep searching
    /// Else, the root is the current working directory, and we do not search higher than the cwd
    fn get_configuration_search_root(&self) -> Option<PathBuf> {
        match self.opt.search_parent_directories {
            true => None,
            false => Some(self.current_directory.to_path_buf()),
        }
    }

    pub fn load_configuration(&mut self, path: &Path) -> Result<Config> {
        if let Some(configuration) = self.forced_configuration {
            return Ok(configuration);
        }

        let root = self.get_configuration_search_root();

        let absolute_path = self.current_directory.join(path);
        let parent_path = &absolute_path
            .parent()
            .with_context(|| format!("no parent directory found for {}", path.display()))?;

        match self.find_config_file(parent_path, root)? {
            Some(config) => Ok(config),
            None => {
                #[cfg(feature = "editorconfig")]
                if self.opt.no_editorconfig {
                    Ok(self.default_configuration)
                } else {
                    editorconfig::parse(self.default_configuration, path)
                        .context("could not parse editorconfig")
                }
                #[cfg(not(feature = "editorconfig"))]
                Ok(self.default_configuration)
            }
        }
    }

    pub fn load_configuration_for_stdin(&mut self) -> Result<Config> {
        if let Some(configuration) = self.forced_configuration {
            return Ok(configuration);
        }

        let root = self.get_configuration_search_root();
        let my_current_directory = self.current_directory.to_owned();

        match &self.opt.stdin_filepath {
            Some(filepath) => self.load_configuration(filepath),
            None => match self.find_config_file(&my_current_directory, root)? {
                Some(config) => Ok(config),
                None => {
                    #[cfg(feature = "editorconfig")]
                    if self.opt.no_editorconfig {
                        Ok(self.default_configuration)
                    } else {
                        editorconfig::parse(self.default_configuration, &PathBuf::from("*.lua"))
                            .context("could not parse editorconfig")
                    }
                    #[cfg(not(feature = "editorconfig"))]
                    Ok(self.default_configuration)
                }
            },
        }
    }

    fn lookup_config_file_in_directory(&self, directory: &Path) -> Result<Option<Config>> {
        debug!("config: looking for config in {}", directory.display());
        let config_file = find_toml_file(directory);
        match config_file {
            Some(file_path) => {
                debug!("config: found config at {}", file_path.display());
                let config = read_and_apply_overrides(&file_path, self.opt)?;
                debug!("config: {:#?}", config);
                Ok(Some(config))
            }
            None => Ok(None),
        }
    }

    /// Looks for a configuration file in the directory provided
    /// Keep searching recursively upwards until we hit the root (if provided), then stop
    /// When `--search-parent-directories` is enabled, root = None, else root = Some(cwd)
    fn find_config_file(
        &mut self,
        directory: &Path,
        root: Option<PathBuf>,
    ) -> Result<Option<Config>> {
        if let Some(config) = self.config_cache.get(directory) {
            return Ok(*config);
        }

        let resolved_configuration = match self.lookup_config_file_in_directory(directory)? {
            Some(config) => Some(config),
            None => {
                let parent_directory = directory.parent();
                let should_stop = Some(directory) == root.as_deref() || parent_directory.is_none();

                if should_stop {
                    debug!("config: no configuration file found");
                    if self.opt.search_parent_directories {
                        if let Some(config) = self.search_config_locations()? {
                            return Ok(Some(config));
                        }
                    }

                    debug!("config: falling back to default config");
                    None
                } else {
                    self.find_config_file(parent_directory.unwrap(), root)?
                }
            }
        };

        self.config_cache
            .insert(directory.to_path_buf(), resolved_configuration);
        Ok(resolved_configuration)
    }

    /// Looks for a configuration file at either `$XDG_CONFIG_HOME`, `$XDG_CONFIG_HOME/stylua`, `$HOME/.config` or `$HOME/.config/stylua`
    fn search_config_locations(&self) -> Result<Option<Config>> {
        // Look in `$XDG_CONFIG_HOME`
        if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
            let xdg_config_path = Path::new(&xdg_config);
            if xdg_config_path.exists() {
                debug!("config: looking in $XDG_CONFIG_HOME");

                if let Some(config) = self.lookup_config_file_in_directory(xdg_config_path)? {
                    return Ok(Some(config));
                }

                debug!("config: looking in $XDG_CONFIG_HOME/stylua");
                let xdg_config_path = xdg_config_path.join("stylua");
                if xdg_config_path.exists() {
                    if let Some(config) = self.lookup_config_file_in_directory(&xdg_config_path)? {
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

                if let Some(config) = self.lookup_config_file_in_directory(&home_config_path)? {
                    return Ok(Some(config));
                }

                debug!("config: looking in $HOME/.config/stylua");
                let home_config_path = home_config_path.join("stylua");
                if home_config_path.exists() {
                    if let Some(config) = self.lookup_config_file_in_directory(&home_config_path)? {
                        return Ok(Some(config));
                    }
                }
            }
        }

        Ok(None)
    }
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

pub fn find_ignore_file_path(mut directory: PathBuf, recursive: bool) -> Option<PathBuf> {
    debug!("config: looking for ignore file in {}", directory.display());
    let file_path = directory.join(".styluaignore");
    if file_path.is_file() {
        debug!("config: resolved ignore file at {}", file_path.display());
        Some(file_path)
    } else if recursive && directory.pop() {
        find_ignore_file_path(directory, recursive)
    } else {
        None
    }
}

/// Handles any overrides provided by command line options
fn load_overrides(config: Config, opt: &Opt) -> Config {
    let mut new_config = config;

    if let Some(syntax) = opt.format_opts.syntax {
        new_config.syntax = syntax.into();
    };
    if let Some(column_width) = opt.format_opts.column_width {
        new_config.column_width = column_width;
    };
    if let Some(line_endings) = opt.format_opts.line_endings {
        new_config.line_endings = line_endings.into();
    };
    if let Some(indent_type) = opt.format_opts.indent_type {
        new_config.indent_type = indent_type.into();
    };
    if let Some(indent_width) = opt.format_opts.indent_width {
        new_config.indent_width = indent_width;
    };
    if let Some(quote_style) = opt.format_opts.quote_style {
        new_config.quote_style = quote_style.into();
    };
    if let Some(call_parentheses) = opt.format_opts.call_parentheses {
        new_config.call_parentheses = call_parentheses.into();
    };
    if let Some(space_after_function_names) = opt.format_opts.space_after_function_names {
        new_config.space_after_function_names = space_after_function_names.into();
    };
    if let Some(collapse_simple_statement) = opt.format_opts.collapse_simple_statement {
        new_config.collapse_simple_statement = collapse_simple_statement.into();
    }
    if opt.format_opts.sort_requires {
        new_config.sort_requires = SortRequiresConfig { enabled: true }
    }

    new_config
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::StructOpt;
    use stylua_lib::{CallParenType, IndentType, LineEndings, LuaVersion, QuoteStyle};

    #[test]
    fn test_override_syntax() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--syntax", "Lua51"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.syntax, LuaVersion::Lua51);
    }

    #[test]
    fn test_override_column_width() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--column-width", "80"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.column_width, 80);
    }

    #[test]
    fn test_override_line_endings() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--line-endings", "Windows"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.line_endings, LineEndings::Windows);
    }

    #[test]
    fn test_override_indent_type() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--indent-type", "Spaces"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.indent_type, IndentType::Spaces);
    }

    #[test]
    fn test_override_indent_width() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--indent-width", "2"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.indent_width, 2);
    }

    #[test]
    fn test_override_quote_style() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--quote-style", "ForceSingle"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.quote_style, QuoteStyle::ForceSingle);
    }

    #[test]
    fn test_override_call_parentheses() {
        let override_opt = Opt::parse_from(vec!["BINARY_NAME", "--call-parentheses", "None"]);
        let default_config = Config::new();
        let config = load_overrides(default_config, &override_opt);
        assert_eq!(config.call_parentheses, CallParenType::None);
    }
}
