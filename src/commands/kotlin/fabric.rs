use std::process::Command;

use anstream::println;
use anstyle::{AnsiColor, Style};
use tracing::info;

use crate::commands::{Generator, kotlin::Kotlin, upgrade::Upgrade, utils::execute_command};

/// Fabric generator
pub(crate) struct Fabric;

impl Generator for Fabric {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Kotlin.docs_path().join("fabric")
    }
}

impl Upgrade for Fabric {
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading Fabric.");
        println!("{msg_style}Upgrading Fabric.{msg_style:#}");
        info!("Upgrading.");

        execute_command(Command::new("fabric.cmd").arg("upgrade"))?;

        info!("Done.");
        info!("Done upgrading Fabric.");

        Ok(())
    }
}
