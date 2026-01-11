use clap::Subcommand;

use crate::commands::{
    Commands, Generator,
    javascript::{JavaScript, tauri::nextjs::NextJS},
};

mod nextjs;

#[derive(Subcommand)]
pub enum TauriCommands {
    /// Generate Tauri NextJS projects.
    #[command(alias = "next")]
    NextJS,
}

impl Commands for TauriCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            TauriCommands::NextJS => Box::new(NextJS::default()),
        }
    }
}

/// Tauri generator
#[derive(Default)]
pub(super) struct Tauri;

impl Generator for Tauri {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        JavaScript::default().docs_path().join("tauri")
    }
}
