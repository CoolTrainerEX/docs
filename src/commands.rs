use std::path::PathBuf;

use anyhow::anyhow;
use clap::Subcommand;

use crate::{
    Commands, Generator, OptionalSubcommands,
    commands::{
        cpp::Cpp,
        go::{Go, GoCommands},
        javascript::{JSCommands, Javascript},
        kotlin::{KTCommands, Kotlin},
        python::Python,
        rust::Rust,
    },
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
    fn generator(self) -> Box<dyn Generator> {
        match self {
            RootCommands::Cpp => Box::new(Cpp::default()),
            RootCommands::Go(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(Go::default()),
            },
            RootCommands::Javascript(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(Javascript::default()),
            },
            RootCommands::Kotlin(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(Kotlin::default()),
            },
            RootCommands::Python => Box::new(Python::default()),
            RootCommands::Rust => Box::new(Rust::default()),
        }
    }
}

#[derive(Default, Debug)]
pub struct Root {}

impl Generator for Root {
    fn generate(&self, _name: String) -> anyhow::Result<()> {
        Err(anyhow!("Select a generator."))
    }

    fn docs_path(&self) -> PathBuf {
        PathBuf::from(".")
    }
}
