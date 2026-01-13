use std::{env, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::Context;
use dialoguer::{Input, theme::ColorfulTheme};
use indicatif::ProgressBar;
use tracing::info;

use crate::{
    DOCS_DIR,
    commands::{
        Generator,
        go::Go,
        upgrade::Upgrade,
        utils::{Deps, execute_command, extract_files},
    },
};

/// Gin generator
#[derive(Debug)]
pub(crate) struct Gin;

impl Generator for Gin {
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(4);

        info!("Generating Gin project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Gin project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(dir = current_dir.to_str());

        let mut go = Go::default();

        go.generate(name)?;
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
            execute_command(Command::new("go").args(["get", "-u"]).args(deps))?;
        }

        if let Some(dev) = deps.dev {
            for dep in dev {
                execute_command(Command::new("go").args(["get", "-tool"]).arg(dep))?;
            }
        }

        bar.inc(1);

        info!("Done.");
        info!("Running init commands.");

        execute_command(
            Command::new("go").args([
                "tool",
                "cobra-cli",
                "init",
                "-a",
                &go.username,
                "-l",
                &Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Liscense")
                    .interact_text()
                    .context("Input error.")?,
                "--viper",
            ]),
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
        Go::default().docs_path().join("gin")
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
