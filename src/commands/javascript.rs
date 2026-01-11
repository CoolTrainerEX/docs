use std::process::Command;

use clap::Subcommand;

use crate::commands::{
    Commands, Generator,
    javascript::{
        nextjs::NextJS,
        tauri::{Tauri, TauriCommands},
    },
    root::Root,
    upgrade::Upgrade,
    utils::{OptionalSubcommands, execute_command},
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
pub(super) struct JavaScript;

impl Generator for JavaScript {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("javascript")
    }
}

impl Upgrade for JavaScript {
    fn upgrade(&self) -> anyhow::Result<()> {
        execute_command(Command::new("deno").arg("clean"))
    }
}
