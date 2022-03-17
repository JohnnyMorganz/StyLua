use anyhow::{bail, Context, Result};
use clap::StructOpt;
use console::style;
use ignore::{overrides::OverrideBuilder, WalkBuilder};
use log::{LevelFilter, *};
use serde_json::json;
use std::fs;
use std::io::{stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use threadpool::ThreadPool;

use stylua_lib::{format_code, Config, OutputVerification, Range};

mod config;
mod opt;
mod output_diff;

static EXIT_CODE: AtomicI32 = AtomicI32::new(0);

enum FormatResult {
    /// Operation was a success, the output was either written to a file or stdout. If diffing, there was no diff to create.
    Complete,
    /// Formatting was a success, but the formatted contents are buffered, ready to be sent to stdout.
    /// This is used when formatting from stdin - we want to buffer the output so it can be sent in a locked channel.
    SuccessBufferedOutput(Vec<u8>),
    /// There is a diff output. This stores the diff created
    Diff(Vec<u8>),
}

fn format_file(
    path: &Path,
    config: Config,
    range: Option<Range>,
    opt: &opt::Opt,
    verify_output: OutputVerification,
) -> Result<FormatResult> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;

    let before_formatting = Instant::now();
    let formatted_contents = format_code(&contents, config, range, verify_output)
        .with_context(|| format!("could not format file {}", path.display()))?;
    let after_formatting = Instant::now();

    debug!(
        "formatted {} in {:?}",
        path.display(),
        after_formatting.duration_since(before_formatting)
    );

    if opt.check {
        let diff = match opt.output_format {
            opt::OutputFormat::Standard => output_diff::output_diff(
                &contents,
                &formatted_contents,
                3,
                &format!("Diff in {}:", path.display()),
                opt.color,
            ),
            opt::OutputFormat::Unified => {
                output_diff::output_diff_unified(&contents, &formatted_contents)
            }
            opt::OutputFormat::Json => {
                output_diff::output_diff_json(&contents, &formatted_contents)
                    .map(|mismatches| {
                        serde_json::to_vec(&json!({
                            "file": path.display().to_string(),
                            "mismatches": mismatches
                        }))
                        // Add newline to end
                        .map(|mut vec| {
                            vec.push(b'\n');
                            vec
                        })
                        // Covert to anyhow::Error
                        .map_err(|err| err.into())
                    })
                    .transpose()
            }
        }
        .context("failed to create diff")?;

        match diff {
            Some(diff) => Ok(FormatResult::Diff(diff)),
            None => Ok(FormatResult::Complete),
        }
    } else {
        fs::write(path, formatted_contents)
            .with_context(|| format!("could not write to {}", path.display()))?;
        Ok(FormatResult::Complete)
    }
}

/// Takes in a string and returns the formatted output in a buffer
/// Used when input has been provided to stdin
fn format_string(
    input: String,
    config: Config,
    range: Option<Range>,
    opt: &opt::Opt,
    verify_output: OutputVerification,
) -> Result<FormatResult> {
    let formatted_contents =
        format_code(&input, config, range, verify_output).context("failed to format from stdin")?;

    if opt.check {
        let diff = output_diff::output_diff(
            &input,
            &formatted_contents,
            3,
            "Diff from stdin:",
            opt.color,
        )
        .context("failed to create diff")?;

        match diff {
            Some(diff) => Ok(FormatResult::Diff(diff)),
            None => Ok(FormatResult::Complete),
        }
    } else {
        Ok(FormatResult::SuccessBufferedOutput(
            formatted_contents.into_bytes(),
        ))
    }
}

fn format(opt: opt::Opt) -> Result<i32> {
    if opt.files.is_empty() {
        bail!("no files provided");
    }

    // Check for incompatible options
    if !opt.check && !matches!(opt.output_format, opt::OutputFormat::Standard) {
        bail!("--output-format can only be used when --check is enabled");
    }

    // Load the configuration
    let config = config::load_config(&opt)?;

    // Handle any configuration overrides provided by options
    let config = config::load_overrides(config, &opt);
    debug!("config: {config:#?}");

    // Create range if provided
    let range = if opt.range_start.is_some() || opt.range_end.is_some() {
        Some(Range::from_values(opt.range_start, opt.range_end))
    } else {
        None
    };

    // Determine if we need to verify the output
    let verify_output = if opt.verify {
        OutputVerification::Full
    } else {
        OutputVerification::None
    };

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

    // Look for an ignore file in the current working directory
    let ignore_path = cwd.join(".styluaignore");
    if ignore_path.is_file() {
        walker_builder.add_ignore(ignore_path);
    }

    let use_default_glob = match opt.glob {
        Some(ref globs) => {
            // Build overriders with any patterns given
            let mut overrides = OverrideBuilder::new(cwd);
            for pattern in globs {
                overrides.add(pattern)?;
            }
            let overrides = overrides.build()?;
            walker_builder.overrides(overrides);
            // We shouldn't use the default glob anymore
            false
        }
        None => true,
    };

    debug!("creating a pool with {} threads", opt.num_threads);
    let pool = ThreadPool::new(std::cmp::max(opt.num_threads, 2)); // Use a minimum of 2 threads, because we need atleast one output reader as well as a formatter
    let (tx, rx) = crossbeam_channel::unbounded();
    let opt = Arc::new(opt);

    // Create a thread to handle the formatting output
    pool.execute(move || {
        for output in rx {
            match output {
                Ok(result) => match result {
                    FormatResult::Complete => (),
                    FormatResult::SuccessBufferedOutput(output) => {
                        let stdout = stdout();
                        let mut handle = stdout.lock();
                        match handle.write_all(&output) {
                            Ok(_) => (),
                            Err(err) => {
                                error!("could not output to stdout: {err:#}")
                            }
                        };
                    }
                    FormatResult::Diff(diff) => {
                        if EXIT_CODE.load(Ordering::SeqCst) != 2 {
                            EXIT_CODE.store(1, Ordering::SeqCst);
                        }

                        let stdout = stdout();
                        let mut handle = stdout.lock();
                        match handle.write_all(&diff) {
                            Ok(_) => (),
                            Err(err) => error!("{err:#}"),
                        }
                    }
                },
                Err(err) => error!("{err:#}"),
            }
        }
    });

    let walker = walker_builder.build();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.is_stdin() {
                    let tx = tx.clone();
                    let opt = opt.clone();

                    pool.execute(move || {
                        let mut buf = String::new();
                        match stdin().read_to_string(&mut buf) {
                            Ok(_) => {
                                tx.send(format_string(buf, config, range, &opt, verify_output))
                            }
                            Err(error) => {
                                tx.send(Err(error).context("could not format from stdin"))
                            }
                        }
                        .unwrap();
                    });
                } else {
                    let path = entry.path().to_owned(); // TODO: stop to_owned?
                    let opt = opt.clone();
                    if path.is_file() {
                        // If the user didn't provide a glob pattern, we should match against our default one
                        // We should ignore the glob check if the path provided was explicitly given to the CLI
                        if use_default_glob && !opt.files.iter().any(|p| path == *p) {
                            lazy_static::lazy_static! {
                                static ref DEFAULT_GLOB: globset::GlobSet = {
                                    let mut builder = globset::GlobSetBuilder::new();
                                    builder.add(globset::Glob::new("**/*.lua").expect("cannot create default glob"));
                                    #[cfg(feature = "luau")]
                                    builder.add(globset::Glob::new("**/*.luau").expect("cannot create default luau glob"));
                                    builder.build().expect("cannot build default globset")
                                };
                            }
                            if !DEFAULT_GLOB.is_match(&path) {
                                continue;
                            }
                        }

                        let tx = tx.clone();
                        pool.execute(move || {
                            tx.send(format_file(&path, config, range, &opt, verify_output))
                                .unwrap()
                        });
                    }
                }
            }
            Err(error) => match error {
                ignore::Error::WithPath { path, err } => match *err {
                    ignore::Error::Io(error) => match error.kind() {
                        std::io::ErrorKind::NotFound => {
                            error!("no file or directory found matching '{:#}'", path.display())
                        }
                        _ => error!("{error:#}"),
                    },
                    _ => error!("{err:#}"),
                },
                _ => error!("{error:#}"),
            },
        }
    }

    drop(tx);
    pool.join();

    // Exit with non-zero code if we have a panic
    let output_code = if pool.panic_count() > 0 {
        2
    } else {
        EXIT_CODE.load(Ordering::SeqCst)
    };
    Ok(output_code)
}

fn main() {
    let opt = opt::Opt::parse();
    let should_use_color = opt.color.should_use_color_stderr();
    let level_filter = if opt.verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Warn
    };

    env_logger::Builder::from_env("STYLUA_LOG")
        .filter(None, level_filter)
        .format(move |buf, record| {
            // Side effect: set exit code
            if let Level::Error = record.level() {
                EXIT_CODE.store(2, Ordering::SeqCst);
            }

            let tag = match record.level() {
                Level::Error => style("error").red(),
                Level::Warn => style("warn").yellow(),
                Level::Info => style("info").green(),
                Level::Debug => style("debug").cyan(),
                Level::Trace => style("trace").magenta(),
            }
            .bold()
            .force_styling(should_use_color);

            writeln!(
                buf,
                "{}{} {}",
                tag,
                style(":").bold().force_styling(should_use_color),
                record.args()
            )
        })
        .init();

    let exit_code = match format(opt) {
        Ok(code) => code,
        Err(err) => {
            error!("{err:#}");
            2
        }
    };

    std::process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_no_files_provided() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.assert()
            .failure()
            .code(2)
            .stderr("error: no files provided\n");
    }

    #[test]
    fn test_format_stdin() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg("-")
            .write_stdin("local   x   = 1")
            .assert()
            .success()
            .stdout("local x = 1\n");
    }
}
