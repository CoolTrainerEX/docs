use clap::Subcommand;

use crate::{
    Commands, Generator,
    commands::javascript::{Javascript, tauri::nextjs::NextJS},
};

mod nextjs;

#[derive(Subcommand)]
pub enum TauriCommands {
    NextJS,
}

impl Commands for TauriCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            TauriCommands::NextJS => Box::new(NextJS::default()),
        }
    }
}

#[derive(Default)]
pub struct Tauri {}

impl Generator for Tauri {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Javascript::default().docs_path().join("tauri")
    }
}
