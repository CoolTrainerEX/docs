use std::{env, fs, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::Context;
use clap::Subcommand;
use dialoguer::{Input, theme::ColorfulTheme};
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{
        Commands, Generator,
        go::gin::Gin,
        root::Root,
        upgrade::Upgrade,
        utils::{Deps, execute_command},
    },
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
#[derive(Debug, Default)]
pub(super) struct Go {
    username: String,
}

impl Generator for Go {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(3);

        info!("Generating Go project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Go project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");

        fs::create_dir_all(&proj_dir).context("Failed to create directory.")?;
        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        self.username = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("User name")
            .interact_text()
            .context("Input error.")?;

        execute_command(Command::new("go").args([
            "mod",
            "init",
            &format!("github.com/{}/{}", self.username, &name),
        ]))?;

        bar.inc(1);

        info!("Done.");
        info!("Running init commands.");

        execute_command(Command::new("docker").arg("init"))?;

        info!("Done.");
        info!("Installing dependencies");

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
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
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
