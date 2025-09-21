use crate::config::find_ignore_file_path;
use crate::opt::Opt;
use anyhow::{Context, Result};
use ignore::gitignore::Gitignore;
use std::path::{Path, PathBuf};

fn get_ignore(
    directory: &Path,
    search_parent_directories: bool,
    search_root: Option<PathBuf>,
) -> Result<Gitignore, ignore::Error> {
    let file_path = find_ignore_file_path(directory.to_path_buf(), search_parent_directories)
        .or_else(|| {
            search_root
                .or_else(|| std::env::current_dir().ok())
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
pub fn is_explicitly_provided(opt: &Opt, path: &Path) -> bool {
    opt.files.iter().any(|p| path == *p)
}

/// By default, files explicitly passed to the command line will be formatted regardless of whether
/// they are present in .styluaignore / not glob matched. If `--respect-ignores` is provided,
/// then we enforce .styluaignore / glob matching on explicitly passed paths.
pub fn should_respect_ignores(opt: &Opt, path: &Path) -> bool {
    !is_explicitly_provided(opt, path) || opt.respect_ignores
}

pub fn path_is_stylua_ignored(
    path: &Path,
    search_parent_directories: bool,
    search_root: Option<PathBuf>,
) -> Result<bool> {
    let ignore = get_ignore(
        path.parent().expect("cannot get parent directory"),
        search_parent_directories,
        search_root,
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
