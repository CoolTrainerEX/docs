use std::{env, fs, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::{Context, Result};
use clap::Subcommand;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{
        Commands, Generator,
        javascript::{
            nextjs::NextJS,
            tauri::{Tauri, TauriCommands},
        },
        root::Root,
        upgrade::Upgrade,
        utils::{Deps, OptionalSubcommands, execute_command},
    },
};

mod nextjs;
mod tauri;

#[derive(Subcommand)]
pub enum JSCommands {
    /// Generate NextJS projects.
    #[command(alias = "next")]
    NextJS,

    /// Generate Tauri projects.
    Tauri(OptionalSubcommands<TauriCommands>),
}

impl Commands for JSCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            JSCommands::NextJS => Box::new(NextJS),
            JSCommands::Tauri(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(Tauri),
            },
        }
    }
}

/// JavaScript generator
#[derive(Debug)]
pub(super) struct JavaScript;

impl Generator for JavaScript {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(3);

        info!("Generating JavaScript project.");

        let proj_dir = env::current_dir()
            .context("Failed to get current directory")?
            .join(&name);

        println!(
            "{msg_style}Generating JavaScript project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");

        fs::create_dir_all(&proj_dir).context("Failed to create directory.")?;
        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        execute_command(Command::new("npm").arg("init"))?;
        bar.inc(1);

        info!("Done.");
        info!("Running init commands.");

        execute_command(Command::new("pnpm").args(["create", "@eslint/config"]))?;
        bar.inc(1);

        info!("Done.");
        info!("Installing dependencies");

        install_js_deps()?;
        bar.inc(1);

        info!("Done.");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("javascript")
    }
}

impl Upgrade for JavaScript {
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading JavaScript.");
        println!("{msg_style}Upgrading JavaScript.{msg_style:#}");
        info!("Upgrading app.");

        execute_command(Command::new("npm").args(["update", "-g"]))?;
        execute_command(Command::new("corepack").args(["install", "-g", "pnpm"]))?;

        info!("Done.");
        info!("Upgrading global tools.");

        execute_command(Command::new("pnpm").args(["update", "-g"]))?;

        info!("Done.");
        info!("Clearing cache.");

        execute_command(Command::new("pnpm").args(["store", "prune"]))?;
        execute_command(Command::new("corepack").args(["cache", "clean"]))?;

        info!("Done.");
        info!("Done upgrading JavaScript.");

        Ok(())
    }
}

/// Runs the JavaScript dependency install commands.
///
/// # Returns
/// Process [`Result`]
fn install_js_deps() -> Result<()> {
    let strong_style = Style::new().bold();

    let deps: Deps = serde_json::from_slice(
        DOCS_DIR
            .get_file(JavaScript.docs_path().join("deps").with_extension("json"))
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

    Ok(())
}
