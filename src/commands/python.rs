use std::{env, path::Path, process::Command};

use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{
        Generator,
        root::Root,
        upgrade::Upgrade,
        utils::{Deps, execute_command, extract_files},
    },
};

/// Python generator
#[derive(Debug)]
pub(super) struct Python;

impl Generator for Python {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(4);

        info!("Generating Python project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Python project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(dir = current_dir.to_str());

        execute_command(Command::new("uv").args(["init", &name]))?;
        bar.inc(1);

        info!("Done.");
        info!("Installing dependencies");

        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        let deps: Deps = serde_json::from_slice(
            DOCS_DIR
                .get_file(self.docs_path().join("deps").with_extension("json"))
                .context(format!(
                    "Cannot find {strong_style}deps.json{strong_style:#}."
                ))?
                .contents(),
        )
        .context(format!(
            "Failed to parse {strong_style}deps.json{strong_style:#}."
        ))?;

        if let Some(deps) = deps.deps {
            execute_command(Command::new("uv").arg("add").args(deps))?;
        }

        if let Some(dev) = deps.dev {
            execute_command(Command::new("uv").arg("add").args(dev).arg("--dev"))?;
        }

        bar.inc(1);

        info!("Done.");
        info!("Running documentation init command.");

        execute_command(
            Command::new(Path::new(".venv").join("Scripts").join("sphinx-quickstart"))
                .args(["./docs/", "--ext-autodoc"]),
        )?;

        bar.inc(1);

        info!("Done.");
        info!("Creating files.");

        extract_files(
            DOCS_DIR
                .get_dir(self.docs_path().join("create"))
                .context("Cannot find create directory.")?,
            &proj_dir,
        )?;

        bar.inc(1);

        info!("Done.");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
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
        info!("Upgrading app.");

        execute_command(Command::new("uv").args(["python", "upgrade"]))?;

        info!("Done.");
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
