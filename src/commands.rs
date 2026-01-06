use clap::Subcommand;

use crate::{
    Commands, Generator, OptionalSubcommands,
    commands::{go::GoCommands, javascript::JSCommands, kotlin::KTCommands},
};

mod cpp;
mod go;
mod javascript;
mod kotlin;
mod python;
mod rust;

#[derive(Subcommand)]
pub enum RootCommands {
    Cpp,
    Go(OptionalSubcommands<GoCommands>),
    #[command(alias = "js")]
    Javascript(OptionalSubcommands<JSCommands>),
    #[command(alias = "kt")]
    Kotlin(OptionalSubcommands<KTCommands>),
    #[command(alias = "py")]
    Python,
    #[command(alias = "rs")]
    Rust,
}

impl Commands for RootCommands {
    fn generator(self) -> impl Generator {
        Root {}
    }
}

pub struct Root {}

impl Generator for Root {
    fn generate(name: Option<String>) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path() -> std::path::PathBuf {
        todo!()
    }
}
