use std::process::Command;

use anstream::println;
use anstyle::{AnsiColor, Style};
use tracing::{info, instrument};

use crate::commands::{Generator, root::Root, upgrade::Upgrade, utils::execute_command};

/// C++ generator
#[derive(Debug)]
pub(super) struct Cpp;

impl Generator for Cpp {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("cpp")
    }
}

impl Upgrade for Cpp {
    #[instrument]
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading C++.");
        println!("{msg_style}Upgrading C++.{msg_style:#}");
        info!("Clearing cache.");

        execute_command(Command::new("conan").args(["cache", "clean"]))?;

        info!("Done.");
        info!("Done upgrading C++.");

        Ok(())
    }
}
