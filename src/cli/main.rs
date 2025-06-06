use anyhow::{bail, Context, Result};
use clap::StructOpt;
use console::style;
use ignore::{gitignore::Gitignore, overrides::OverrideBuilder, WalkBuilder};
use log::{LevelFilter, *};
use serde_json::json;
use std::collections::HashSet;
use std::fs;
use std::io::{stderr, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;
use thiserror::Error;
use threadpool::ThreadPool;

use stylua_lib::{format_code, Config, OutputVerification, Range};

use crate::config::find_ignore_file_path;

mod config;
mod opt;
mod output_diff;

static EXIT_CODE: AtomicI32 = AtomicI32::new(0);
static UNFORMATTED_FILE_COUNT: AtomicU32 = AtomicU32::new(0);

enum FormatResult {
    /// Operation was a success, the output was either written to a file or stdout. If diffing, there was no diff to create.
    Complete,
    /// Formatting was a success, but the formatted contents are buffered, ready to be sent to stdout.
    /// This is used when formatting from stdin - we want to buffer the output so it can be sent in a locked channel.
    SuccessBufferedOutput(Vec<u8>),
    /// There is a diff output. This stores the diff created
    Diff(Vec<u8>),
}

/// Wraps an error to include information about the file it resonated from
#[derive(Error, Debug)]
#[error("{:#}", .error)]
struct ErrorFileWrapper {
    file: String,
    error: anyhow::Error,
}

fn convert_parse_error_to_json(file: &str, errs: Vec<full_moon::Error>) -> serde_json::Value {
    errs.iter()
        .map(|err| {
            let message = match err {
                full_moon::Error::AstError(ast_error) => format!(
                    "unexpected token `{}`: {}",
                    ast_error.token(),
                    ast_error.error_message()
                ),
                full_moon::Error::TokenizerError(error) => match error.error() {
                    full_moon::tokenizer::TokenizerErrorType::UnclosedComment => {
                        "unclosed comment".to_string()
                    }
                    full_moon::tokenizer::TokenizerErrorType::UnclosedString => {
                        "unclosed string".to_string()
                    }
                    full_moon::tokenizer::TokenizerErrorType::InvalidNumber => {
                        "invalid number".to_string()
                    }
                    full_moon::tokenizer::TokenizerErrorType::UnexpectedToken(character) => {
                        format!("unexpected character {character}")
                    }
                    full_moon::tokenizer::TokenizerErrorType::InvalidSymbol(symbol) => {
                        format!("invalid symbol {symbol}")
                    }
                },
            };
            let (start_position, end_position) = err.range();
            json!({
                "type": "parse_error",
                "message": message,
                "filename": file,
                "location": {
                    "start": start_position.bytes(),
                    "start_line": start_position.line(),
                    "start_column": start_position.character(),
                    "end": end_position.bytes(),
                    "end_line": end_position.line(),
                    "end_column": end_position.character(),
                },
            })
        })
        .collect()
}

fn create_diff(
    opt: &opt::Opt,
    original: &str,
    expected: &str,
    file_name: &str,
) -> Result<Option<Vec<u8>>> {
    match opt.output_format {
        opt::OutputFormat::Standard => output_diff::output_diff(
            original,
            expected,
            3,
            &format!("Diff in {file_name}:"),
            opt.color,
        ),
        opt::OutputFormat::Unified => output_diff::output_diff_unified(original, expected),
        opt::OutputFormat::Json => {
            output_diff::output_diff_json(original, expected)
                .map(|mismatches| {
                    serde_json::to_vec(&json!({
                        "file": file_name,
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
        opt::OutputFormat::Summary => {
            if original == expected {
                Ok(None)
            } else {
                Ok(Some(format!("{file_name}\n").into_bytes()))
            }
        }
    }
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
        let diff = create_diff(
            opt,
            &contents,
            &formatted_contents,
            path.display().to_string().as_str(),
        )
        .context("failed to create diff")?;

        match diff {
            Some(diff) => Ok(FormatResult::Diff(diff)),
            None => Ok(FormatResult::Complete),
        }
    } else {
        if formatted_contents != contents {
            fs::write(path, formatted_contents)
                .with_context(|| format!("could not write to {}", path.display()))?;
        }
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
    should_skip: bool,
) -> Result<FormatResult> {
    let formatted_contents = if should_skip {
        input.clone()
    } else {
        format_code(&input, config, range, verify_output).context("failed to format from stdin")?
    };

    if opt.check {
        let diff = create_diff(opt, &input, &formatted_contents, "stdin")
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

fn get_ignore(
    directory: &Path,
    search_parent_directories: bool,
) -> Result<Gitignore, ignore::Error> {
    let file_path = find_ignore_file_path(directory.to_path_buf(), search_parent_directories)
        .or_else(|| {
            std::env::current_dir()
                .ok()
                .and_then(|cwd| find_ignore_file_path(cwd, false))
        });

    if let Some(file_path) = file_path {
        let (ignore, err) = Gitignore::new(file_path);
        if let Some(err) = err {
            Err(err)
        } else {
            Ok(ignore)
        }
    } else {
        Ok(Gitignore::empty())
    }
}

/// Whether the provided path was explicitly provided to the tool
fn is_explicitly_provided(opt: &opt::Opt, path: &Path) -> bool {
    opt.files.iter().any(|p| path == *p)
}

/// By default, files explicitly passed to the command line will be formatted regardless of whether
/// they are present in .styluaignore / not glob matched. If `--respect-ignores` is provided,
/// then we enforce .styluaignore / glob matching on explicitly passed paths.
fn should_respect_ignores(opt: &opt::Opt, path: &Path) -> bool {
    !is_explicitly_provided(opt, path) || opt.respect_ignores
}

fn path_is_stylua_ignored(path: &Path, search_parent_directories: bool) -> Result<bool> {
    let ignore = get_ignore(
        path.parent().expect("cannot get parent directory"),
        search_parent_directories,
    )
    .context("failed to parse ignore file")?;

    // matched_path_or_any_parents panics when path is not in cwd
    // can happen when `--respect-ignores --stdin-filepath {path}`
    if !path
        .canonicalize()
        .unwrap_or_default()
        .starts_with(ignore.path().canonicalize().unwrap_or_default())
    {
        return Ok(false);
    }

    Ok(matches!(
        ignore.matched_path_or_any_parents(path, false),
        ignore::Match::Ignore(_)
    ))
}

fn format(opt: opt::Opt) -> Result<i32> {
    debug!("resolved options: {:#?}", opt);

    if opt.files.is_empty() {
        bail!("no files provided");
    }

    // Check for incompatible options
    if !opt.check
        && matches!(
            opt.output_format,
            opt::OutputFormat::Unified | opt::OutputFormat::Summary
        )
    {
        bail!("--output-format=unified and --output-format=summary can only be used when --check is enabled");
    }

    // Load the configuration
    let opt_for_config_resolver = opt.clone();
    let mut config_resolver = config::ConfigResolver::new(&opt_for_config_resolver)?;

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
        .standard_filters(true)
        .hidden(!opt.allow_hidden)
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
    let pool = ThreadPool::new(std::cmp::max(opt.num_threads, 2)); // Use a minimum of 2 threads, because we need at least one output reader as well as a formatter
    let (tx, rx) = crossbeam_channel::unbounded::<Result<_>>();
    let output_format = opt.output_format;
    let opt = Arc::new(opt);

    // Output a header if in summary mode
    if matches!(opt.output_format, opt::OutputFormat::Summary) {
        println!(
            "{} Checking formatting...",
            style("!")
                .cyan()
                .bold()
                .force_styling(opt.color.should_use_color())
        );
    }

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
                                error!("could not output to stdout: {:#}", err)
                            }
                        };
                    }
                    FormatResult::Diff(diff) => {
                        if EXIT_CODE.load(Ordering::SeqCst) != 2 {
                            EXIT_CODE.store(1, Ordering::SeqCst);
                        }

                        UNFORMATTED_FILE_COUNT.fetch_add(1, Ordering::SeqCst);

                        let stdout = stdout();
                        let mut handle = stdout.lock();
                        match handle.write_all(&diff) {
                            Ok(_) => (),
                            Err(err) => error!("{:#}", err),
                        }
                    }
                },
                Err(err) if matches!(output_format, opt::OutputFormat::Json) => {
                    match err.downcast_ref::<ErrorFileWrapper>() {
                        Some(ErrorFileWrapper { file, error }) => {
                            match error.downcast_ref::<stylua_lib::Error>() {
                                Some(stylua_lib::Error::ParseError(err)) => {
                                    let structured_err =
                                        convert_parse_error_to_json(file, err.to_vec());
                                    // Force write to stderr directly
                                    // TODO: can we do this through error! instead?
                                    let stderr = stderr();
                                    let mut handle = stderr.lock();
                                    match handle.write_all(structured_err.to_string().as_bytes()) {
                                        Ok(_) => (),
                                        Err(err) => {
                                            error!("could not output to stdout: {:#}", err)
                                        }
                                    };
                                }
                                _ => error!("{:#}", err),
                            }
                        }
                        _ => error!("{:#}", err),
                    }
                }
                Err(err) => error!("{:#}", err),
            }
        }
    });

    let walker = walker_builder.build();
    let mut seen_files = HashSet::new();

    for result in walker {
        match result {
            Ok(entry) => {
                if entry.is_stdin() {
                    let tx = tx.clone();
                    let opt = opt.clone();

                    let should_skip_format = match &opt.stdin_filepath {
                        Some(path) => {
                            opt.respect_ignores
                                && path_is_stylua_ignored(path, opt.search_parent_directories)?
                        }
                        None => false,
                    };

                    let config = config_resolver.load_configuration_for_stdin()?;

                    pool.execute(move || {
                        let mut buf = String::new();
                        tx.send(
                            stdin()
                                .read_to_string(&mut buf)
                                .map_err(|err| err.into())
                                .and_then(|_| {
                                    format_string(
                                        buf,
                                        config,
                                        range,
                                        &opt,
                                        verify_output,
                                        should_skip_format,
                                    )
                                    .context("could not format from stdin")
                                })
                                .map_err(|error| {
                                    ErrorFileWrapper {
                                        file: "stdin".to_string(),
                                        error,
                                    }
                                    .into()
                                }),
                        )
                        .unwrap()
                    });
                } else {
                    let path = entry.path().to_owned(); // TODO: stop to_owned?
                    let opt = opt.clone();

                    if seen_files.contains(&path) {
                        continue;
                    }
                    seen_files.insert(path.clone());

                    if path.is_file() {
                        // If the user didn't provide a glob pattern, we should match against our default one
                        if use_default_glob && should_respect_ignores(opt.as_ref(), path.as_path())
                        {
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

                        // If `--respect-ignores` was given and this is an explicit file path,
                        // we should check .styluaignore
                        if is_explicitly_provided(opt.as_ref(), &path)
                            && should_respect_ignores(opt.as_ref(), &path)
                            && path_is_stylua_ignored(&path, opt.search_parent_directories)?
                        {
                            continue;
                        }

                        let config = config_resolver.load_configuration(&path)?;

                        let tx = tx.clone();
                        pool.execute(move || {
                            tx.send(
                                format_file(&path, config, range, &opt, verify_output).map_err(
                                    |error| {
                                        ErrorFileWrapper {
                                            file: path.display().to_string(),
                                            error,
                                        }
                                        .into()
                                    },
                                ),
                            )
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
                        _ => error!("{:#}", error),
                    },
                    _ => error!("{:#}", err),
                },
                _ => error!("{:#}", error),
            },
        }
    }

    drop(tx);
    pool.join();

    // Output summary

    if matches!(opt.output_format, opt::OutputFormat::Summary) {
        let file_count = UNFORMATTED_FILE_COUNT.load(Ordering::SeqCst);
        if file_count == 0 {
            println!(
                "{} All files are correctly formatted.",
                style("✓")
                    .green()
                    .bold()
                    .force_styling(opt.color.should_use_color())
            );
        } else {
            println!(
                "{} Code style issues found in {} file{} above.",
                style("✕")
                    .red()
                    .bold()
                    .force_styling(opt.color.should_use_color()),
                style(file_count)
                    .yellow()
                    .bold()
                    .force_styling(opt.color.should_use_color()),
                if file_count == 1 { "" } else { "s" }
            );
        }
    }

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
    let output_format = opt.output_format;
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

            if let opt::OutputFormat::Json = output_format {
                writeln!(
                    buf,
                    "{}",
                    json!({
                        "type": record.level().to_string().to_lowercase(),
                        "message": record.args().to_string(),
                    })
                )
            } else {
                writeln!(
                    buf,
                    "{}{} {}",
                    tag,
                    style(":").bold().force_styling(should_use_color),
                    record.args()
                )
            }
        })
        .init();

    let exit_code = match format(opt) {
        Ok(code) => code,
        Err(err) => {
            error!("{:#}", err);
            2
        }
    };

    std::process::exit(exit_code);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use assert_fs::prelude::*;

    macro_rules! construct_tree {
        ({ $($file_name:literal:$file_contents:literal,)* }) => {{
            let cwd = assert_fs::TempDir::new().unwrap();

            $(
                cwd.child($file_name).write_str($file_contents).unwrap();
            )*

            cwd
        }};
    }

    fn create_stylua() -> Command {
        Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap()
    }

    #[test]
    fn test_no_files_provided() {
        let mut cmd = create_stylua();
        cmd.assert()
            .failure()
            .code(2)
            .stderr("error: no files provided\n");
    }

    #[test]
    fn test_format_stdin() {
        let mut cmd = create_stylua();
        cmd.arg("-")
            .write_stdin("local   x   = 1")
            .assert()
            .success()
            .stdout("local x = 1\n");
    }

    #[test]
    fn test_format_file() {
        let cwd = construct_tree!({
            "foo.lua": "local   x    =   1",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path()).arg(".").assert().success();

        cwd.child("foo.lua").assert("local x = 1\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_stylua_ignore() {
        let cwd = construct_tree!({
            ".styluaignore": "ignored/",
            "foo.lua": "local   x    =   1",
            "ignored/bar.lua": "local   x    =   1",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path()).arg(".").assert().success();

        cwd.child("foo.lua").assert("local x = 1\n");
        cwd.child("ignored/bar.lua").assert("local   x    =   1");

        cwd.close().unwrap();
    }

    #[test]
    fn explicitly_provided_files_dont_check_ignores() {
        let cwd = construct_tree!({
            ".styluaignore": "foo.lua",
            "foo.lua": "local   x    =   1",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .arg("foo.lua")
            .assert()
            .success();

        cwd.child("foo.lua").assert("local x = 1\n");

        cwd.close().unwrap();
    }

    #[test]
    fn explicitly_provided_files_dont_check_ignores_stdin() {
        let cwd = construct_tree!({
            ".styluaignore": "foo.lua",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--stdin-filepath", "foo.lua", "-"])
            .write_stdin("local   x    =   1")
            .assert()
            .success()
            .stdout("local x = 1\n");

        cwd.close().unwrap();
    }

    #[test]
    fn explicitly_provided_files_not_in_cwd() {
        let cwd = construct_tree!({
            ".styluaignore": "foo.lua",
        });

        let another = construct_tree!({});

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args([
                "--respect-ignores",
                "--stdin-filepath",
                another.child("foo.lua").to_str().unwrap(),
                "-",
            ])
            .write_stdin("local   x    =   1")
            .assert()
            .success()
            .stdout("local x = 1\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_respect_ignores() {
        let cwd = construct_tree!({
            ".styluaignore": "foo.lua",
            "foo.lua": "local   x    =   1",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--respect-ignores", "foo.lua"])
            .assert()
            .success();

        cwd.child("foo.lua").assert("local   x    =   1");

        cwd.close().unwrap();
    }

    #[test]
    fn test_respect_ignores_stdin() {
        let cwd = construct_tree!({
            ".styluaignore": "foo.lua",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--respect-ignores", "--stdin-filepath", "foo.lua", "-"])
            .write_stdin("local   x    =   1")
            .assert()
            .success()
            .stdout("local   x    =   1");

        cwd.close().unwrap();
    }

    #[test]
    fn test_respect_ignores_directory_no_glob() {
        // https://github.com/JohnnyMorganz/StyLua/issues/845
        let cwd = construct_tree!({
            ".styluaignore": "build/",
            "build/foo.lua": "local   x    =   1",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--check", "--respect-ignores", "build/foo.lua"])
            .assert()
            .success();

        cwd.close().unwrap();
    }

    #[test]
    fn test_stdin_filepath_respects_cwd_configuration_next_to_file() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--stdin-filepath", "foo.lua", "-"])
            .write_stdin("local x = \"hello\"")
            .assert()
            .success()
            .stdout("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_stdin_filepath_respects_cwd_configuration_for_nested_file() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--stdin-filepath", "build/foo.lua", "-"])
            .write_stdin("local x = \"hello\"")
            .assert()
            .success()
            .stdout("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_cwd_configuration_respected_when_formatting_from_stdin() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .arg("-")
            .write_stdin("local x = \"hello\"")
            .assert()
            .success()
            .stdout("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_cwd_configuration_respected_for_file_in_cwd() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .arg("foo.lua")
            .assert()
            .success();

        cwd.child("foo.lua").assert("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_cwd_configuration_respected_for_nested_file() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .arg("build/foo.lua")
            .assert()
            .success();

        cwd.child("build/foo.lua").assert("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_configuration_is_not_used_outside_of_cwd() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.child("build").path())
            .arg("foo.lua")
            .assert()
            .success();

        cwd.child("build/foo.lua").assert("local x = \"hello\"\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_configuration_used_outside_of_cwd_when_search_parent_directories_is_enabled() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.child("build").path())
            .args(["--search-parent-directories", "foo.lua"])
            .assert()
            .success();

        cwd.child("build/foo.lua").assert("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_configuration_is_searched_next_to_file() {
        let cwd = construct_tree!({
            "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .arg("build/foo.lua")
            .assert()
            .success();

        cwd.child("build/foo.lua").assert("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_configuration_is_used_closest_to_the_file() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferDouble'",
            "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
            "build/foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .arg("build/foo.lua")
            .assert()
            .success();

        cwd.child("build/foo.lua").assert("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_respect_config_path_override() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferDouble'",
            "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--config-path", "build/stylua.toml", "foo.lua"])
            .assert()
            .success();
    }

    #[test]
    fn test_respect_config_path_override_for_stdin_filepath() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferDouble'",
            "build/stylua.toml": "quote_style = 'AutoPreferSingle'",
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--config-path", "build/stylua.toml", "-"])
            .write_stdin("local x = \"hello\"")
            .assert()
            .success()
            .stdout("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_uses_cli_overrides_instead_of_default_configuration() {
        let cwd = construct_tree!({
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--quote-style", "AutoPreferSingle", "."])
            .assert()
            .success();

        cwd.child("foo.lua").assert("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_uses_cli_overrides_instead_of_default_configuration_stdin_filepath() {
        let cwd = construct_tree!({
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--quote-style", "AutoPreferSingle", "-"])
            .write_stdin("local x = \"hello\"")
            .assert()
            .success()
            .stdout("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_uses_cli_overrides_instead_of_found_configuration() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferDouble'",
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--quote-style", "AutoPreferSingle", "."])
            .assert()
            .success();

        cwd.child("foo.lua").assert("local x = 'hello'\n");

        cwd.close().unwrap();
    }

    #[test]
    fn test_uses_cli_overrides_instead_of_found_configuration_stdin_filepath() {
        let cwd = construct_tree!({
            "stylua.toml": "quote_style = 'AutoPreferDouble'",
            "foo.lua": "local x = \"hello\"",
        });

        let mut cmd = create_stylua();
        cmd.current_dir(cwd.path())
            .args(["--quote-style", "AutoPreferSingle", "-"])
            .write_stdin("local x = \"hello\"")
            .assert()
            .success()
            .stdout("local x = 'hello'\n");

        cwd.close().unwrap();
    }
}
