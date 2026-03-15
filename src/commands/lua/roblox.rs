use std::{env, fs, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{
        Generator,
        lua::Lua,
        upgrade::Upgrade,
        utils::{execute_command, extract_files},
    },
};

/// Roblox generator
#[derive(Debug)]
pub(crate) struct Roblox;

impl Generator for Roblox {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(3);

        info!("Generating Roblox project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Roblox project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");

        fs::create_dir_all(&proj_dir).context("Failed to create directory.")?;
        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        execute_command(Command::new("rokit").arg("init"))?;
        bar.inc(1);

        info!("Done.");
        info!("Running init commands.");

        for dep in [
            "rojo-rbx/rojo",
            "UpliftGames/wally",
            "rojo-rbx/run-in-roblox",
        ] {
            execute_command(Command::new("rokit").arg("add").arg(dep))?;
        }

        execute_command(Command::new("rojo").arg("init"))?;
        execute_command(Command::new("wally").arg("init"))?;
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
        Lua.docs_path().join("roblox")
    }
}

impl Upgrade for Roblox {
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading Roblox.");
        println!("{msg_style}Upgrading Roblox.{msg_style:#}");
        info!("Upgrading.");

        execute_command(Command::new("rokit").arg("self-update"))?;

        info!("Done.");
        info!("Done upgrading Roblox.");

        Ok(())
    }
}
