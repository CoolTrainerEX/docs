use std::process::Command;

use anstyle::{AnsiColor, Style};
use tracing::info;

use crate::commands::{Generator, root::Root, upgrade::Upgrade, utils::execute_command};

/// Python generator
pub(super) struct Python;

impl Generator for Python {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("python")
    }
}

impl Upgrade for Python {
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading Python.");
        println!("{msg_style}Upgrading Python.{msg_style:#}");
        info!("Upgrading tools.");

        execute_command(Command::new("uv").args(["tool", "upgrade", "--all"]))?;

        info!("Done.");
        info!("Clearing cache.");

        execute_command(Command::new("uv").args(["cache", "prune"]))?;

        info!("Done.");
        info!("Done upgrading Python.");

        Ok(())
    }
}
