use std::{ffi::OsStr, fs, iter, path::Path, process::Command};

use anstyle::Style;
use anyhow::{Context, Result, anyhow};
use clap::{Args, Subcommand};
use include_dir::Dir;
use serde::Deserialize;

/// Subcommands wrapped with [`Option`]
#[derive(Args)]
pub struct OptionalSubcommands<T: Subcommand> {
    /// Optional command.
    #[command(subcommand)]
    pub command: Option<T>,
}

/// `deps.json` structure in documentation.
#[derive(Deserialize)]
pub(super) struct Deps {
    /// List of dependencies.
    pub(super) deps: Option<Vec<String>>,

    /// List of dev dependencies.
    pub(super) dev: Option<Vec<String>>,
}

/// Helper function to run commands.
///
/// # Parameters
/// - `command` - [`Command`] to run
///
/// # Returns
/// Process [`Result`]
pub(super) fn execute_command(command: &mut Command) -> Result<()> {
    let strong_style = Style::new().bold();
    let cmd_str = iter::once(command.get_program())
        .chain(command.get_args())
        .collect::<Vec<_>>()
        .join(OsStr::new(" "));

    let status = command.status().context(format!(
        "Failed to execute {strong_style}{}{strong_style:#}.",
        cmd_str.display()
    ))?;

    status.success().then_some(()).ok_or(anyhow!(
        "Process {strong_style}{}{strong_style:#} failed. ({})",
        cmd_str.display(),
        status
    ))
}

/// Extracts files from [`Dir`], removing any path prefix.
///
/// # Parameters
/// - `dir` - [`Dir`] to extract
/// - `base_path` - Path to extract to
///
/// # Returns
/// Process [`Result`]
pub(super) fn extract_files(dir: &Dir, base_path: impl AsRef<Path>) -> Result<()> {
    extract_files_root(dir, base_path, dir.path())
}

/// Extracts files from [`Dir`], removing any path prefix.
///
/// # Parameters
/// - `dir` - [`Dir`] to extract
/// - `base_path` - Path to extract to
/// - `root` - Root [`Dir`] path
///
/// # Returns
/// Process [`Result`]
fn extract_files_root(dir: &Dir, base_path: impl AsRef<Path>, root: &Path) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(dir) => extract_files_root(dir, base_path.as_ref(), root)?,
            include_dir::DirEntry::File(file) => {
                let dest_path = base_path.as_ref().join(
                    file.path()
                        .strip_prefix(root)
                        .context("Failed to get file path.")?,
                );

                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent).context("Failed to create directories.")?;
                }

                fs::write(dest_path, file.contents()).context("Failed to create file.")?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_fs::{TempDir, assert::PathAssert, prelude::PathChild};
    use include_dir::include_dir;
    use predicates::path;

    use super::*;

    #[test]
    fn test_extract_files() -> Result<()> {
        let temp = TempDir::new().context("Failed to generate temp directory.")?;
        let dir = include_dir!("test")
            .get_dir("test_dir")
            .context("Failed to get test directory.")?;

        extract_files(dir, &temp)?;

        temp.child(Path::new("test1").with_extension("txt"))
            .assert("test1");
        temp.child(Path::new("dir").join("test2").with_extension("txt"))
            .assert("test2");
        temp.child(Path::new("test3").with_extension("txt"))
            .assert(path::missing());

        temp.close().context("Failed to close temp directory.")
    }
}
