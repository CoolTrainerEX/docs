use clap::Subcommand;

use crate::{
    Commands, Generator,
    commands::{
        OptionalSubcommands, Root,
        javascript::{
            nextjs::NextJS,
            tauri::{Tauri, TauriCommands},
        },
    },
};

mod nextjs;
mod tauri;

#[derive(Subcommand)]
pub enum JSCommands {
    #[command(alias = "next")]
    NextJS,
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

#[derive(Default)]
pub struct Javascript {}

impl Generator for Javascript {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("javascript")
    }
}
