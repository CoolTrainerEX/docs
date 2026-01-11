use std::path::PathBuf;

use anyhow::Result;
use clap::Subcommand;

use crate::commands::root::RootCommands;

mod cpp;
mod go;
mod javascript;
mod kotlin;
mod python;
pub mod root;
mod rust;
pub mod upgrade;
pub mod utils;

/// Commands enum trait
pub trait Commands {
    /// Gets the generator for the command to execute.
    ///
    /// # Returns
    /// - The generator function
    fn generator(self) -> Box<dyn Generator>;
}

/// Project generators
pub trait Generator {
    /// Generate the project.
    ///
    /// # Parameters
    /// - `name` - Name value
    ///
    /// # Returns
    /// - Process [`Result`]
    fn generate(&self, name: String) -> Result<()>;

    /// Returns the relative documentation path
    ///
    /// # Returns
    /// - Documentation path
    fn docs_path(&self) -> PathBuf;
}

#[derive(Subcommand)]
pub enum Subcommands {
    #[command(flatten)]
    Generate(RootCommands),

    /// Run upgrades. Uses Scoop.
    Upgrade,
}
