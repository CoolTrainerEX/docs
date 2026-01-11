use std::process::Command;

use anstream::println;
use anstyle::{AnsiColor, Style};
use tracing::info;

use crate::commands::{Generator, go::Go, upgrade::Upgrade, utils::execute_command};

/// Gin generator
#[derive(Debug)]
pub(crate) struct Gin;

impl Generator for Gin {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Go.docs_path().join("gin")
    }
}

impl Upgrade for Gin {
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading Gin.");
        println!("{msg_style}Upgrading Gin.{msg_style:#}");
        info!("Upgrading documentation tool.");

        execute_command(Command::new("deno").args([
            "install",
            "-A",
            "-g",
            "-f",
            "npm:@redocly/cli",
        ]))?;

        info!("Done.");
        info!("Done upgrading Gin.");

        Ok(())
    }
}
