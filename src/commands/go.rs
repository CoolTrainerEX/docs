use clap::Subcommand;

use crate::{
    Commands, Generator,
    commands::{Root, go::gin::Gin},
};

mod gin;

#[derive(Subcommand)]
pub enum GoCommands {
    Gin,
}

impl Commands for GoCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            GoCommands::Gin => Box::new(Gin::default()),
        }
    }
}

#[derive(Default)]
pub struct Go {}

impl Generator for Go {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("go")
    }
}
