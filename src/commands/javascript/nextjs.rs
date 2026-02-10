use std::{env, process::Command};

use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{
        Generator,
        javascript::{JavaScript, install_js_deps},
        utils::{Deps, execute_command, extract_files},
    },
};

/// NextJS generator
#[derive(Debug)]
pub(super) struct NextJS;

impl Generator for NextJS {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(4);

        info!("Generating NextJS project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating NextJS project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(dir = current_dir.to_str());

        execute_command(Command::new("pnpm").args(["create", "next-app", &name]))?;
        bar.inc(1);

        info!("Done.");
        info!("Running init commands.");

        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        execute_command(Command::new("pnpx").args(["shadcn", "init"]))?;
        execute_command(Command::new("pnpm").args(["create", "playwright"]))?;
        bar.inc(1);

        info!("Done.");
        info!("Installing dependencies");

        install_js_deps()?;

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
            execute_command(Command::new("pnpm").arg("add").args(deps))?;
        }

        if let Some(dev) = deps.dev {
            execute_command(Command::new("pnpm").args(["add", "-D"]).args(dev))?;
        }

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
        JavaScript.docs_path().join("nextjs")
    }
}
