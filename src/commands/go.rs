use std::process::Command;

use anstream::println;
use anstyle::{AnsiColor, Style};
use clap::Subcommand;
use tracing::info;

use crate::commands::{
    Commands, Generator, go::gin::Gin, root::Root, upgrade::Upgrade, utils::execute_command,
};

pub(super) mod gin;

#[derive(Subcommand)]
pub enum GoCommands {
    /// Generate Gin projects.
    Gin,
}

impl Commands for GoCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            GoCommands::Gin => Box::new(Gin),
        }
    }
}

/// Go generator
#[derive(Debug)]
pub(super) struct Go;

impl Generator for Go {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("go")
    }
}

impl Upgrade for Go {
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading Go.");
        println!("{msg_style}Upgrading Go.{msg_style:#}");
        info!("Clearing cache.");

        execute_command(Command::new("go").args([
            "clean",
            "-cache",
            "-testcache",
            "-modcache",
            "-fuzzcache",
        ]))?;

        info!("Done.");
        info!("Done upgrading Go.");

        Ok(())
    }
}
