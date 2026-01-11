use std::{ffi::OsStr, iter, path::PathBuf, process::Command};

use anstyle::Style;
use anyhow::{Context, Result, anyhow};
use clap::{Args, Subcommand};

mod cpp;
mod go;
mod javascript;
mod kotlin;
mod python;
pub mod root;
mod rust;

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
