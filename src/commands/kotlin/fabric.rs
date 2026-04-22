use std::{env, path::Path, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::commands::{Generator, kotlin::Kotlin, upgrade::Upgrade, utils::execute_command};

/// Fabric generator
#[derive(Debug)]
pub(crate) struct Fabric;

impl Generator for Fabric {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(2);

        info!("Generating Fabric project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Fabric project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(dir = current_dir.to_str());

        execute_command(Command::new("fabric.cmd").args(["init", &name]))?;
        bar.inc(1);

        info!("Done.");
        info!("Running init commands.");

        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        let gradlew_path = Path::new(".").join("gradlew");

        execute_command(Command::new(&gradlew_path).arg("vscode"))?;
        execute_command(Command::new(&gradlew_path).arg("genSources"))?;
        bar.inc(1);

        info!("Done.");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Kotlin.docs_path().join("fabric")
    }
}
