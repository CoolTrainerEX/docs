use clap::Subcommand;

use crate::{
    Commands, Generator,
    commands::{Root, kotlin::fabric::Fabric},
};

mod fabric;

/// Kotlin generator
///
/// Generator returns an error.
#[derive(Subcommand)]
pub enum KTCommands {
    /// Generate Fabric projects.
    Fabric,
}

impl Commands for KTCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            KTCommands::Fabric => Box::new(Fabric::default()),
        }
    }
}

#[derive(Default)]
pub(super) struct Kotlin;

impl Generator for Kotlin {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("kotlin")
    }
}
