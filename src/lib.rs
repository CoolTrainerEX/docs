use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

pub mod commands;

/// Subcommands wrapped with [`Option`]
#[derive(Args)]
pub struct OptionalSubcommands<T: Subcommand> {
    #[command(subcommand)]
    pub command: Option<T>,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
