use anyhow::{format_err, Result};
use luafmt_lib::{format_code, Config};
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "luafmt", about = "A utility to format Lua code")]
struct Opt {
    /// Specify path to luafmt.toml
    #[structopt(long = "config-path", parse(from_os_str))]
    config_path: Option<PathBuf>,

    /// A glob pattern to test against which files to check
    #[structopt(long, default_value = "**/*.lua")]
    pattern: String,

    /// A list of files to format
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn format_file(path: &PathBuf, config: Config) -> Result<()> {
    match fs::read(path) {
        Ok(contents) => {
            let contents = String::from_utf8_lossy(&contents);
            let formatted_contents = match format_code(&contents, config) {
                Ok(formatted) => formatted,
                Err(error) => { return Err(format_err!("error: could not format file {}: {}", path.display(), error)) }
            };

            match fs::write(path, formatted_contents) {
                Ok(_) => Ok(()),
                Err(error) => Err(format_err!("error: could not write to file {}: {}", path.display(), error))
            }
        },
        Err(error) => {
            Err(format_err!("error: could not open file {}: {}", path.display(), error))
        }
    }
}

fn format(opt: Opt) -> Result<i32> {
    if opt.files.is_empty() {
        return Err(format_err!("error: no files provided"));
    }

    let config: Config = match opt.config_path {
        Some(config_path) => match fs::read_to_string(config_path) {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(error) => {
                    return Err(format_err!("error: config file not in correct format: {}", error));
                }
            },
            Err(error) => {
                return Err(format_err!("error: couldn't read config file: {}", error));
            }
        }

        None => match fs::read_to_string("luafmt.toml") {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(error) => {
                    return Err(format_err!("error: config file not in correct format: {}", error));
                }
            }

            Err(_) => Config::default()
        }
    };

    let mut errors = vec![];

    for file_path in opt.files.iter() {
        if file_path.exists() {
            if file_path.is_file() {
                match format_file(file_path, config) {
                    Ok(_) => continue,
                    Err(error) => errors.push(error)
                }
            } else if file_path.is_dir() {
                let glob_pattern = format!("{}/{}", file_path.to_string_lossy(), opt.pattern);
                match glob::glob(&glob_pattern) {
                    Ok(entries) => {
                        for entry in entries {
                            match entry {
                                Ok(path) => {
                                    match format_file(&path, config) {
                                        Ok(_) => continue,
                                        Err(error) => errors.push(error)
                                    }
                                },
                                Err(error) => errors.push(format_err!("error: failed to read file {}", error))
                            }
                        }
                    },
                    Err(error) => errors.push(format_err!("error: failed to read glob pattern {}: {}", glob_pattern, error))
                }
            } else {
                errors.push(format_err!("error: unknown path {}", file_path.display()))
            }
        } else {
            errors.push(format_err!("error: file {} not found", file_path.display()));
            continue;
        }
    }

    if errors.len() > 0 {
        for error in errors.iter() {
            eprintln!("{}", error.to_string());
        }
        return Ok(1)
    }

    Ok(0)
}

fn main() {
    let opt = Opt::from_args();

    let exit_code = match format(opt) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{}", e.to_string());
            1
        }
    };

    std::process::exit(exit_code);
}
