use std::{env, process::Command};

use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::commands::{
    Generator,
    javascript::{JavaScript, install_js_deps},
    utils::execute_command,
};

/// Tauri generator
#[derive(Debug)]
pub(super) struct Tauri;

impl Generator for Tauri {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(2);

        info!("Generating Tauri project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Tauri project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(dir = current_dir.to_str());

        execute_command(Command::new("pnpm").args(["create", "tauri-app", &name]))?;
        bar.inc(1);

        info!("Done.");
        info!("Installing dependencies");

        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        install_js_deps()?;
        bar.inc(1);

        info!("Done.");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> std::path::PathBuf {
        JavaScript.docs_path().join("tauri")
    }
}
