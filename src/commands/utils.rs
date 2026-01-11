use std::{ffi::OsStr, iter, process::Command};

use anstyle::Style;
use anyhow::{Context, Result, anyhow};
use clap::{Args, Subcommand};

/// Subcommands wrapped with [`Option`]
#[derive(Args)]
pub struct OptionalSubcommands<T: Subcommand> {
    #[command(subcommand)]
    pub command: Option<T>,
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
