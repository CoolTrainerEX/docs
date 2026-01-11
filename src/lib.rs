use std::{env, ffi::OsStr, iter, path::PathBuf, process::Command};

use anstyle::{AnsiColor, Style};
use anyhow::{Context, Result, anyhow};
use clap::{Args, Subcommand};
use include_dir::include_dir;
use tracing::{info, instrument};

use crate::commands::Root;

const DEFAULT_CONFIG_DIR: include_dir::Dir<'_> = include_dir!("config");
const DOCS_DIR: include_dir::Dir<'_> = include_dir!("docs");

/// Gets the application config directory.
///
/// # Returns
/// - The config directory
pub fn config_dir() -> PathBuf {
    env::home_dir().unwrap_or(PathBuf::from("/")).join(".docs")
}

/// Returns the path to the default config file.
///
/// # Returns
/// - Path to the default config file
pub fn default_config() -> PathBuf {
    config_dir().join("config").with_extension("toml")
}

/// Checks if [`config_dir`] exists. Uses existence of [`default_config`] to check.
///
/// # Returns
/// - Process [`Result`]
#[instrument]
pub fn config_dir_exists_or_gen() -> Result<()> {
    let error_style = Style::new().fg_color(Some(AnsiColor::Red.into())).bold();

    if default_config()
        .try_exists()
        .context("Cannot access config directory.")?
    {
        info!("Config directory found.");
        Ok(())
    } else {
        info!("Config directory not found.");
        eprintln!("{error_style}Config directory does not exist.{error_style:#}");
        Root::default().generate(String::new())
    }
}

// Commands

pub mod commands;

/// Subcommands wrapped with [`Option`]
#[derive(Args)]
pub struct OptionalSubcommands<T: Subcommand> {
    #[command(subcommand)]
    pub command: Option<T>,
}

/// Commands enum trait
pub trait Commands {
    /// Gets the generator for the command to execute.
    ///
    /// # Returns
    /// - The generator function
    fn generator(self) -> Box<dyn Generator>;
}

/// Project generators
pub trait Generator {
    /// Generate the project.
    ///
    /// # Parameters
    /// - `name` - Name value
    ///
    /// # Returns
    /// - Process [`Result`]
    fn generate(&self, name: String) -> Result<()>;

    /// Returns the relative documentation path
    ///
    /// # Returns
    /// - Documentation path
    fn docs_path(&self) -> PathBuf;
}

fn execute_command(command: &mut Command) -> Result<()> {
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

#[cfg(test)]
mod tests {

    use std::ffi::OsStr;

    use super::*;

    #[test]
    fn test_config_dir() {
        assert_eq!(config_dir().file_name(), Some(OsStr::new(".docs")));
    }

    #[test]
    fn test_default_config() {
        assert_eq!(
            default_config(),
            config_dir().join("config").with_extension("toml")
        );
    }

    #[test]
    fn test_config_dir_exists_or_gen() {
        todo!();
    }
}
