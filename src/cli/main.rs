use anyhow::{format_err, Result};
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::fs;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use structopt::StructOpt;

use stylua_lib::{format_code, Config, Range};

mod config;
mod opt;
mod output_diff;

fn format_file(
    path: &Path,
    config: Config,
    range: Option<Range>,
    check_only: bool,
    color: opt::Color,
) -> Result<i32> {
    match fs::read(path) {
        Ok(contents) => {
            let contents = String::from_utf8_lossy(&contents);
            let formatted_contents = match format_code(&contents, config, range) {
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
                let is_diff = output_diff::output_diff(
                    &contents,
                    &formatted_contents,
                    3,
                    format!("Diff in {}:", path.display()),
                    color,
                );
                if is_diff {
                    Ok(1)
                } else {
                    Ok(0)
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
fn format_string(input: String, config: Config, range: Option<Range>) -> Result<()> {
    let out = &mut stdout();
    let formatted_contents = match format_code(&input, config, range) {
        Ok(formatted) => formatted,
        Err(error) => return Err(format_err!("error: could not format from stdin: {}", error)),
    };

    match out.write_all(&formatted_contents.into_bytes()) {
        Ok(()) => Ok(()),
        Err(error) => Err(format_err!("error: could not output to stdout: {}", error)),
    }
}

fn format(opt: opt::Opt) -> Result<i32> {
    if opt.files.is_empty() {
        return Err(format_err!("error: no files provided"));
    }

    // Load the configuration
    let config = config::load_config(&opt)?;

    // Create range if provided
    let range = if opt.range_start.is_some() || opt.range_end.is_some() {
        Some(Range::from_values(opt.range_start, opt.range_end))
    } else {
        None
    };

    let mut errors = vec![];
    let mut error_code = 0;

    let cwd = std::env::current_dir()?;

    // Build WalkBuilder with the files given, using any overrides set
    let mut walker_builder = WalkBuilder::new(&opt.files[0]);
    for file_path in &opt.files[1..] {
        walker_builder.add(file_path);
    }

    walker_builder
        .standard_filters(false)
        .hidden(true)
        .parents(true)
        .add_custom_ignore_filename(".styluaignore");

    let use_default_glob = match opt.glob {
        Some(globs) => {
            // Build overriders with any patterns given
            let mut overrides = OverrideBuilder::new(cwd);
            for pattern in globs {
                match overrides.add(&pattern) {
                    Ok(_) => continue,
                    Err(err) => errors.push(format_err!(
                        "error: cannot parse glob pattern {}: {}",
                        pattern,
                        err
                    )),
                }
            }
            let overrides = overrides.build()?;
            walker_builder.overrides(overrides);
            // We shouldn't use the default glob anymore
            false
        }
        None => true,
    };

    let walker = walker_builder.build();

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
                        Ok(_) => match format_string(buf, config, range) {
                            Ok(_) => continue,
                            Err(error) => errors.push(error),
                        },
                        Err(error) => {
                            errors.push(format_err!("error: could not read from stdin: {}", error))
                        }
                    }
                } else {
                    let path = entry.path();
                    if path.is_file() {
                        // If the user didn't provide a glob pattern, we should match against our default one
                        if use_default_glob {
                            lazy_static::lazy_static! {
                                static ref DEFAULT_GLOB: globset::GlobMatcher = globset::Glob::new("**/*.lua").expect("cannot create default glob").compile_matcher();
                            }
                            if !DEFAULT_GLOB.is_match(path) {
                                continue;
                            }
                        }
                        match format_file(path, config, range, opt.check, opt.color) {
                            Ok(code) => {
                                if code != 0 {
                                    error_code = code
                                }
                            }
                            Err(error) => errors.push(error),
                        }
                    }
                }
            }
            Err(error) => {
                errors.push(format_err!("error: could not walk: {}", error));
            }
        }
    }

    if !errors.is_empty() {
        for error in errors.iter() {
            eprintln!("{:#}", error);
        }
        return Ok(1);
    }

    Ok(error_code)
}

fn main() {
    let opt = opt::Opt::from_args();

    let exit_code = match format(opt) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("{:#}", e);
            1
        }
    };

    std::process::exit(exit_code);
}
