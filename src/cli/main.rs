use anyhow::{format_err, Result};
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use stylua_lib::{format_code, Config};
use ignore::{WalkBuilder, overrides::OverrideBuilder};

#[derive(StructOpt, Debug)]
#[structopt(name = "stylua", about = "A utility to format Lua code")]
struct Opt {
    /// Specify path to stylua.toml
    #[structopt(long = "config-path", parse(from_os_str))]
    config_path: Option<PathBuf>,

    /// A glob pattern to test against which files to check
    #[structopt(long, default_value = "**/*.lua")]
    pattern: String,

    /// A list of files to format
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn format_file(path: &Path, config: Config) -> Result<()> {
    match fs::read(path) {
        Ok(contents) => {
            let contents = String::from_utf8_lossy(&contents);
            let formatted_contents = match format_code(&contents, config) {
                Ok(formatted) => formatted,
                Err(error) => {
                    return Err(format_err!(
                        "error: could not format file {}: {}",
                        path.display(),
                        error
                    ))
                }
            };

            match fs::write(path, formatted_contents) {
                Ok(_) => Ok(()),
                Err(error) => Err(format_err!(
                    "error: could not write to file {}: {}",
                    path.display(),
                    error
                )),
            }
        }
        Err(error) => Err(format_err!(
            "error: could not open file {}: {}",
            path.display(),
            error
        )),
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
                    return Err(format_err!(
                        "error: config file not in correct format: {}",
                        error
                    ));
                }
            },
            Err(error) => {
                return Err(format_err!("error: couldn't read config file: {}", error));
            }
        },

        None => match fs::read_to_string("stylua.toml") {
            Ok(contents) => match toml::from_str(&contents) {
                Ok(config) => config,
                Err(error) => {
                    return Err(format_err!(
                        "error: config file not in correct format: {}",
                        error
                    ));
                }
            },

            Err(_) => Config::default(),
        },
    };

    let mut errors = vec![];

    for file_path in opt.files.iter() {
        let override_builder = match OverrideBuilder::new(file_path).add(&opt.pattern) {
            Ok(builder) => builder,
            Err(error) => {
                errors.push(format_err!(
                    "error: failed to parse pattern into glob: {} {}",
                    opt.pattern,
                    error
                ));
                continue
            },
        }.build().unwrap(); // TODO: don't unwrap like this

        let walker = WalkBuilder::new(file_path).standard_filters(false).hidden(true).overrides(override_builder).build();

        for result in walker {
            match result {
                Ok(entry) => {
                    if entry.is_stdin() {
                        // TODO: Handle stdin
                    } else {
                        let path = entry.path();
                        if path.is_file() {
                            match format_file(path, config) {
                                Ok(_) => continue,
                                Err(error) => errors.push(error)
                            }
                        } else {
                            errors.push(format_err!("error: {} is not a file", path.display()));
                        }
                    }
                },
                Err(error) => {
                    errors.push(format_err!("error: could not walk: {}", error));
                }
            }
        }
    }

    if !errors.is_empty() {
        for error in errors.iter() {
            eprintln!("{}", error.to_string());
        }
        return Ok(1);
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
