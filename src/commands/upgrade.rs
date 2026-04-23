use std::process::Command;

use anstyle::{AnsiColor, Style};
use anyhow::Result;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::commands::{
    cpp::Cpp, go::Go, javascript::JavaScript, python::Python, rust::Rust, utils::execute_command,
};

/// Upgraders
pub(super) trait Upgrade {
    /// Run upgrade commands.
    ///
    /// # Returns
    /// Process [`Result`]
    fn upgrade(&self) -> Result<()>;
}

/// Returns a list of [`Upgrade`] to run.
///
/// # Returns
/// List of [`Upgrade`]
fn upgrades() -> Vec<Box<dyn Upgrade>> {
    vec![
        Box::new(Cpp),
        Box::new(Go::default()),
        Box::new(JavaScript),
        Box::new(Python),
        Box::new(Rust),
    ]
}

/// Run upgrade commands. Uses [Nix](https://nixos.org/)
///
/// # Returns
/// Process [`Result`]
#[instrument]
pub fn upgrade() -> Result<()> {
    let upgrades = upgrades();

    let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
    let bar = ProgressBar::new(2 + upgrades.len() as u64);

    info!("Upgrading.");
    println!("{msg_style}Upgrading.{msg_style:#}");
    info!("Upgrading apps.");

    execute_command(Command::new("nix").args(["profile", "upgrade", "--all"]))?;
    execute_command(Command::new("sudo").args(["apt", "update"]))?;
    execute_command(Command::new("sudo").args(["apt", "upgrade"]))?;
    bar.inc(1);

    info!("Done.");
    info!("Clearing cache.");

    execute_command(Command::new("nix-collect-garbage").arg("-d"))?;
    execute_command(Command::new("sudo").args(["apt", "clean"]))?;
    execute_command(Command::new("sudo").args(["apt", "autoremove"]))?;
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
