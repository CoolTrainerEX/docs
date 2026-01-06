use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Subcommand};

pub mod commands;

/// Subcommands wrapped with [`Option`]
#[derive(Args)]
pub struct OptionalSubcommands<T: Subcommand> {
    #[command(subcommand)]
    command: Option<T>,
}

/// Commands enum trait
trait Commands {
    /// Gets the generator for each enum value.
    ///
    /// # Returns
    /// - The [`Generator`] for the enum value
    fn generator(self) -> impl Generator;
}

/// Project generators
pub trait Generator {
    /// Generate the project.
    ///
    /// # Parameters
    /// - `name` - Optional name value. Will prompt if [`None`].
    ///
    /// # Returns
    /// - Process [`Result`]
    fn generate(name: Option<String>) -> Result<()>;

    /// Opens the documentation for the project generation.
    ///
    /// # Returns
    /// - Process [`Result`]
    fn docs_path() -> PathBuf;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
