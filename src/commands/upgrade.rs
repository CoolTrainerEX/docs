use std::process::Command;

use anstyle::{AnsiColor, Style};
use anyhow::Result;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::commands::{
    cpp::Cpp,
    go::{Go, gin::Gin},
    javascript::JavaScript,
    kotlin::fabric::Fabric,
    python::Python,
    utils::execute_command,
};

/// Upgraders
pub(super) trait Upgrade {
    /// Run upgrade commands.
    ///
    /// # Returns
    /// - Process [`Result`]
    fn upgrade(&self) -> Result<()>;
}

/// Returns a list of [`Upgrade`] to run.
///
/// # Returns
/// - List of [`Upgrade`]
fn upgrades() -> Vec<Box<dyn Upgrade>> {
    vec![
        Box::new(Cpp),
        Box::new(Go),
        Box::new(Gin),
        Box::new(JavaScript),
        Box::new(Fabric),
        Box::new(Python),
    ]
}

/// Run upgrade commands. Uses [Scoop](https://scoop.sh/).
///
/// # Returns
/// - Process [`Result`]
#[instrument]
pub fn upgrade() -> Result<()> {
    let upgrades = upgrades();

    let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
    let bar = ProgressBar::new(2 + upgrades.len() as u64);

    info!("Upgrading.");
    println!("{msg_style}Upgrading.{msg_style:#}");
    info!("Upgrading Scoop apps.");

    execute_command(Command::new("scoop.cmd").args(["update", "-a"]))?;
    bar.inc(1);

    info!("Done.");
    info!("Clearing cache.");

    execute_command(Command::new("scoop.cmd").args(["cleanup", "-a", "-k"]))?;
    bar.inc(1);

    info!("Done.");

    for upgrade in upgrades {
        upgrade.upgrade()?;
        bar.inc(1);
    }

    info!("Done upgrading.");

    bar.finish_and_clear();

    Ok(())
}
