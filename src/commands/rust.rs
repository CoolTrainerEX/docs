use std::{env, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use tracing::info;

use crate::{Generator, commands::Root, execute_command};

/// Rust generator
#[derive(Default)]
pub(super) struct Rust;

impl Generator for Rust {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();

        let bar = ProgressBar::new(2);

        info!("Generating project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(path = current_dir.to_str());

        execute_command(Command::new("cargo").args(["new", &name]))?;
        bar.inc(1);

        info!("Done");
        info!("Installing dependencies");
        info!(path = proj_dir.to_str());

        // TODO
        bar.inc(1);

        info!("Done");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("rust")
    }
}
