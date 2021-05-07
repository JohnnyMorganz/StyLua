use crate::opt::Opt;
use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use stylua_lib::Config;

fn find_config_file(opt: &Opt, mut path: PathBuf) -> Result<Option<Config>> {
    // Set the file name
    path.set_file_name("stylua.toml");

    if path.exists() {
        let contents = fs::read_to_string(path).context("Failed to read config file")?;
        let config = toml::from_str(&contents).context("Config file not in correct format")?;

        Ok(Some(config))
    } else {
        // Check for hidden version
        path.set_file_name(".stylua.toml");
        if path.exists() {
            let contents = fs::read_to_string(path).context("Failed to read config file")?;
            let config = toml::from_str(&contents).context("Config file not in correct format")?;

            Ok(Some(config))
        } else {
            // Both don't exist, search up the tree if necessary
            // path.pop() mutates the path to get its parent, and returns false if no more parent
            if opt.search_parent_directories && path.pop() {
                find_config_file(opt, path)
            } else {
                Ok(None)
            }
        }
    }
}

pub fn load_config(opt: &Opt) -> Result<Config> {
    match &opt.config_path {
        Some(config_path) => {
            let contents = fs::read_to_string(config_path).context("Failed to read config file")?;
            let config = toml::from_str(&contents).context("Config file not in correct format")?;

            Ok(config)
        }
        None => {
            let current_dir = env::current_dir().context("Could not find current directory")?;
            let config = find_config_file(opt, current_dir)?;
            match config {
                Some(config) => Ok(config),
                None => Ok(Config::default()),
            }
        }
    }
}
