use anyhow::{format_err, Result};
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::fs;
use std::io::{stdin, stdout, Read, Write};
use std::path::{Path, PathBuf};
use structopt::{clap::arg_enum, StructOpt};
use stylua_lib::{format_code, Config};

mod output_diff;

#[derive(StructOpt, Debug)]
#[structopt(name = "stylua", about = "A utility to format Lua code")]
struct Opt {
    /// Specify path to stylua.toml
    #[structopt(long = "config-path", parse(from_os_str))]
    config_path: Option<PathBuf>,

    /// Runs in 'check' mode
    /// Exits with 0 if all formatting is OK
    /// Exits with 1 if the formatting is incorrect
    /// Any files input will not be overwritten
    #[structopt(short, long)]
    check: bool,

    // Whether the output should include terminal colour or not
    #[structopt(
        long,
        possible_values = &Color::variants(),
        case_insensitive = true,
        default_value = "auto",
    )]
    color: Color,

    /// A glob pattern to test against which files to check
    #[structopt(long, default_value = "**/*.lua")]
    pattern: String,

    /// A list of files to format
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

structopt::clap::arg_enum! {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Color {
        Always,
        Auto,
        Never,
    }
}

impl Color {
    /// Whether we should use a coloured terminal.
    pub fn use_colored_tty(self) -> bool {
        match self {
            Color::Always | Color::Auto => true,
            Color::Never => false,
        }
    }
}

fn format_file(path: &Path, config: Config, check_only: bool, color: Color) -> Result<i32> {
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

            if check_only {
                let diff = output_diff::make_diff(&contents, &formatted_contents, 3);
                if diff.is_empty() {
                    Ok(0)
                } else {
                    output_diff::print_diff(
                        diff,
                        |line| format!("Diff in {} at line {}:", path.display(), line),
                        color,
                    );
                    Ok(1)
                }
            } else {
                match fs::write(path, formatted_contents) {
                    Ok(_) => Ok(0),
                    Err(error) => Err(format_err!(
                        "error: could not write to file {}: {}",
                        path.display(),
                        error
                    )),
                }
            }
        }
        Err(error) => Err(format_err!(
            "error: could not open file {}: {}",
            path.display(),
            error
        )),
    }
}

/// Takes in a string and outputs the formatted version to stdout
/// Used when input has been provided to stdin
fn format_string(input: String, config: Config) -> Result<()> {
    let out = &mut stdout();
    let formatted_contents = match format_code(&input, config) {
        Ok(formatted) => formatted,
        Err(error) => return Err(format_err!("error: could not format from stdin: {}", error)),
    };

    match out.write_all(&formatted_contents.into_bytes()) {
        Ok(()) => Ok(()),
        Err(error) => return Err(format_err!("error: could not output to stdout: {}", error)),
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
    let mut error_code = 0;

    for file_path in opt.files.iter() {
        let override_builder = match OverrideBuilder::new(file_path).add(&opt.pattern) {
            Ok(builder) => builder,
            Err(error) => {
                errors.push(format_err!(
                    "error: failed to parse pattern into glob: {} {}",
                    opt.pattern,
                    error
                ));
                continue;
            }
        }
        .build()
        .unwrap(); // TODO: don't unwrap like this

        let walker = WalkBuilder::new(file_path)
            .standard_filters(false)
            .hidden(true)
            .overrides(override_builder)
            .build();

        for result in walker {
            match result {
                Ok(entry) => {
                    if entry.is_stdin() {
                        if opt.check {
                            errors.push(format_err!(
                                "warning: `--check` cannot be used whilst reading from stdin"
                            ))
                        };

                        let mut buf = String::new();
                        match stdin().read_to_string(&mut buf) {
                            Ok(_) => match format_string(buf, config) {
                                Ok(_) => continue,
                                Err(error) => errors.push(error),
                            },
                            Err(error) => errors
                                .push(format_err!("error: could not read from stdin: {}", error)),
                        }
                    } else {
                        let path = entry.path();
                        if path.is_file() {
                            match format_file(path, config, opt.check, opt.color) {
                                Ok(code) => {
                                    if code != 0 {
                                        error_code = code
                                    }
                                }
                                Err(error) => errors.push(error),
                            }
                        } else {
                            errors.push(format_err!("error: {} is not a file", path.display()));
                        }
                    }
                }
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

    Ok(error_code)
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
