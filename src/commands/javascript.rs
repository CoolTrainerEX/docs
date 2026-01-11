use clap::Subcommand;

use crate::commands::{
    Commands, Generator, OptionalSubcommands,
    javascript::{
        nextjs::NextJS,
        tauri::{Tauri, TauriCommands},
    },
    root::Root,
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
            JSCommands::NextJS => Box::new(NextJS::default()),
            JSCommands::Tauri(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(Tauri::default()),
            },
        }
    }
}

/// JavaScript generator
#[derive(Default)]
pub(super) struct JavaScript;

impl Generator for JavaScript {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("javascript")
    }
}
