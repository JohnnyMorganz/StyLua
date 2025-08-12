use crate::{
    CallParenType, CollapseSimpleStatement, Config, Error, IndentType, LineEndings, LuaVersion,
    QuoteStyle, SortRequiresConfig, SpaceAfterFunctionNames,
};
use log::debug;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(feature = "editorconfig")]
use crate::editorconfig;

static CONFIG_FILE_NAME: [&str; 2] = ["stylua.toml", ".stylua.toml"];

fn read_config_file(path: &Path) -> Result<Config, Error> {
    let contents = fs::read_to_string(path)
        .map_err(|_| Error::ConfigResolutionError("Failed to read config file".to_string()))?;
    let config = toml::from_str(&contents).map_err(|_| {
        Error::ConfigResolutionError("Config file not in correct format".to_string())
    })?;

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

/// Those are configuration values forced by CLI arguments.
#[derive(Default, Debug, Clone)]
pub struct CliConfig {
    pub forced_config_path: Option<PathBuf>,
    pub stdin_filepath: Option<PathBuf>,
    pub search_parent_directories: bool,
    #[cfg(feature = "editorconfig")]
    pub no_editorconfig: bool,

    // Formatting options
    /// The type of Lua syntax to parse
    pub syntax: Option<LuaVersion>,
    /// The column width to use to attempt to wrap lines.
    pub column_width: Option<usize>,
    /// The type of line endings to use.
    pub line_endings: Option<LineEndings>,
    /// The type of indents to use.
    pub indent_type: Option<IndentType>,
    /// The width of a single indentation level.
    pub indent_width: Option<usize>,
    /// The style of quotes to use in string literals.
    pub quote_style: Option<QuoteStyle>,
    /// Specify whether to apply parentheses on function calls with single string or table arg.
    pub call_parentheses: Option<CallParenType>,
    /// Specify whether to collapse simple statements.
    pub collapse_simple_statement: Option<CollapseSimpleStatement>,
    /// Enable requires sorting
    pub sort_requires: bool,
    pub space_after_function_names: Option<SpaceAfterFunctionNames>,
}

impl CliConfig {
    fn read_and_apply_overrides(&self, path: &Path) -> Result<Config, Error> {
        Ok(self.apply_overrides(&read_config_file(path)?))
    }

    fn apply_overrides(&self, config: &Config) -> Config {
        let mut new_config = *config;

        if let Some(syntax) = self.syntax {
            new_config.syntax = syntax;
        }
        if let Some(column_width) = self.column_width {
            new_config.column_width = column_width;
        }
        if let Some(line_endings) = self.line_endings {
            new_config.line_endings = line_endings;
        }
        if let Some(indent_type) = self.indent_type {
            new_config.indent_type = indent_type;
        }
        if let Some(indent_width) = self.indent_width {
            new_config.indent_width = indent_width;
        }
        if let Some(quote_style) = self.quote_style {
            new_config.quote_style = quote_style;
        }
        if let Some(call_parentheses) = self.call_parentheses {
            new_config.call_parentheses = call_parentheses;
        }
        if let Some(collapse_simple_statement) = self.collapse_simple_statement {
            new_config.collapse_simple_statement = collapse_simple_statement;
        }
        if self.sort_requires {
            new_config.sort_requires = SortRequiresConfig { enabled: true }
        }
        if let Some(space_after_function_names) = self.space_after_function_names {
            new_config.space_after_function_names = space_after_function_names;
        };

        new_config
    }
}

pub struct ConfigResolver {
    forced_configuration: Option<Config>,
    current_directory: PathBuf,
    default_configuration: Config,
    cli_config: CliConfig,
}

impl ConfigResolver {
    pub fn new(cli_config: CliConfig) -> Result<ConfigResolver, Error> {
        Ok(ConfigResolver {
            forced_configuration: cli_config
                .forced_config_path
                .as_ref()
                .map(|path| cli_config.read_and_apply_overrides(path))
                .transpose()?,
            current_directory: env::current_dir().map_err(|_| {
                Error::ConfigResolutionError("Could not find current directory".to_string())
            })?,
            default_configuration: cli_config.apply_overrides(&Config::default()),
            cli_config,
        })
    }

    /// Returns the root used when searching for configuration
    /// If `--search-parent-directories`, then there is no root, and we keep searching
    /// Else, the root is the current working directory, and we do not search higher than the cwd
    fn get_configuration_search_root(&self) -> Option<PathBuf> {
        match self.cli_config.search_parent_directories {
            true => None,
            false => Some(self.current_directory.to_path_buf()),
        }
    }

    pub fn load_configuration(&self, path: &Path) -> Result<Config, Error> {
        if let Some(configuration) = self.forced_configuration {
            return Ok(configuration);
        }

        let root = self.get_configuration_search_root();

        let absolute_path = self.current_directory.join(path);
        let parent_path = &absolute_path.parent().ok_or_else(|| {
            Error::ConfigResolutionError(format!(
                "no parent directory found for {}",
                path.display()
            ))
        })?;

        match self.find_config_file(parent_path, root)? {
            Some(config) => Ok(config),
            None => {
                #[cfg(feature = "editorconfig")]
                if self.cli_config.no_editorconfig {
                    Ok(self.default_configuration)
                } else {
                    editorconfig::parse(self.default_configuration, path).map_err(|_| {
                        Error::ConfigResolutionError("could not parse editorconfig".to_string())
                    })
                }
                #[cfg(not(feature = "editorconfig"))]
                Ok(self.default_configuration)
            }
        }
    }

    pub fn load_configuration_for_stdin(&self) -> Result<Config, Error> {
        if let Some(configuration) = self.forced_configuration {
            return Ok(configuration);
        }

        let root = self.get_configuration_search_root();
        let my_current_directory = self.current_directory.to_owned();

        match self.cli_config.stdin_filepath.as_ref() {
            Some(filepath) => self.load_configuration(filepath),
            None => match self.find_config_file(&my_current_directory, root)? {
                Some(config) => Ok(config),
                None => {
                    #[cfg(feature = "editorconfig")]
                    if self.cli_config.no_editorconfig {
                        Ok(self.default_configuration)
                    } else {
                        editorconfig::parse(self.default_configuration, &PathBuf::from("*.lua"))
                            .map_err(|_| {
                                Error::ConfigResolutionError(
                                    "could not parse editorconfig".to_string(),
                                )
                            })
                    }
                    #[cfg(not(feature = "editorconfig"))]
                    Ok(self.default_configuration)
                }
            },
        }
    }

    fn lookup_config_file_in_directory(&self, directory: &Path) -> Result<Option<Config>, Error> {
        debug!("config: looking for config in {}", directory.display());
        let config_file = find_toml_file(directory);
        match config_file {
            Some(file_path) => {
                debug!("config: found config at {}", file_path.display());
                let config = self.cli_config.read_and_apply_overrides(&file_path)?;
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
        &self,
        directory: &Path,
        root: Option<PathBuf>,
    ) -> Result<Option<Config>, Error> {
        let resolved_configuration = match self.lookup_config_file_in_directory(directory)? {
            Some(config) => Some(config),
            None => {
                let parent_directory = directory.parent();
                let should_stop = Some(directory) == root.as_deref() || parent_directory.is_none();

                if should_stop {
                    debug!("config: no configuration file found");
                    if self.cli_config.search_parent_directories {
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

        Ok(resolved_configuration)
    }

    /// Looks for a configuration file at either `$XDG_CONFIG_HOME`, `$XDG_CONFIG_HOME/stylua`, `$HOME/.config` or `$HOME/.config/stylua`
    fn search_config_locations(&self) -> Result<Option<Config>, Error> {
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
