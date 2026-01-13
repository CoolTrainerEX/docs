use std::{env, process::Command};

use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{
        Generator,
        javascript::{self, tauri::Tauri},
        utils::{Deps, execute_command},
    },
};

/// Tauri NextJS generator
#[derive(Debug)]
pub(super) struct NextJS;

impl Generator for NextJS {
    #[instrument]
    fn generate(&self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(3);

        info!("Generating Tauri NextJS project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Tauri NextJS project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(dir = current_dir.to_str());

        javascript::NextJS.generate(name)?;
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
            execute_command(
                Command::new("deno")
                    .arg("add")
                    .args(deps)
                    .arg("--allow-scripts"),
            )?;
        }

        if let Some(dev) = deps.dev {
            execute_command(
                Command::new("deno")
                    .arg("add")
                    .args(dev)
                    .args(["--allow-scripts", "-D"]),
            )?;
        }

        bar.inc(1);

        info!("Done.");
        info!("Running init command.");

        execute_command(Command::new("deno").args(["x", "tauri", "init"]))?;
        bar.inc(1);

        info!("Done.");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Tauri.docs_path().join("nextjs")
    }
}
