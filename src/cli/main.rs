use anyhow::{bail, format_err, Context, Result};
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::fs;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::time::Instant;
use structopt::StructOpt;

use stylua_lib::{format_code, Config, Range};

mod config;
mod opt;
mod output_diff;

#[macro_export]
macro_rules! verbose_println {
    ($verbosity:expr, $str:expr) => {
        if $verbosity {
            println!($str);
        }
    };
    ($verbosity:expr, $str:expr, $($arg:tt)*) => {
        if $verbosity {
            println!($str, $($arg)*);
        }
    };
}

fn format_file(path: &Path, config: Config, range: Option<Range>, opt: &opt::Opt) -> Result<i32> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;

    let before_formatting = Instant::now();
    let formatted_contents = format_code(&contents, config, range)
        .with_context(|| format!("Could not format file {}", path.display()))?;
    let after_formatting = Instant::now();

    verbose_println!(
        opt.verbose,
        "formatted {} in {:?}",
        path.display(),
        after_formatting.duration_since(before_formatting)
    );

    if opt.check {
        let is_diff = output_diff::output_diff(
            &contents,
            &formatted_contents,
            3,
            format!("Diff in {}:", path.display()),
            opt.color,
        );
        Ok(if is_diff { 1 } else { 0 })
    } else {
        fs::write(path, formatted_contents)
            .with_context(|| format!("Could not write to {}", path.display()))?;
        Ok(0)
    }
}

/// Takes in a string and outputs the formatted version to stdout
/// Used when input has been provided to stdin
fn format_string(input: String, config: Config, range: Option<Range>) -> Result<()> {
    let out = &mut stdout();
    let formatted_contents =
        format_code(&input, config, range).context("Failed to format from stdin")?;
    out.write_all(&formatted_contents.into_bytes())
        .context("Could not output to stdout")?;
    Ok(())
}

fn format(opt: opt::Opt) -> Result<i32> {
    if opt.files.is_empty() {
        bail!("error: no files provided");
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
        Some(ref globs) => {
            // Build overriders with any patterns given
            let mut overrides = OverrideBuilder::new(cwd);
            for pattern in globs {
                match overrides.add(pattern) {
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
                        // We should ignore the glob check if the path provided was explicitly given to the CLI
                        if use_default_glob && !opt.files.iter().any(|p| path == *p) {
                            lazy_static::lazy_static! {
                                static ref DEFAULT_GLOB: globset::GlobMatcher = globset::Glob::new("**/*.lua").expect("cannot create default glob").compile_matcher();
                            }
                            if !DEFAULT_GLOB.is_match(path) {
                                continue;
                            }
                        }
                        match format_file(path, config, range, &opt) {
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
