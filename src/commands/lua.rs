use anyhow::anyhow;
use clap::Subcommand;

use crate::commands::{Commands, Generator, lua::roblox::Roblox, root::Root};

pub(super) mod roblox;

#[derive(Subcommand)]
pub enum LuaCommands {
    /// Generate Roblox projects.
    Roblox,
}

impl Commands for LuaCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            LuaCommands::Roblox => Box::new(Roblox),
        }
    }
}

/// Lua generator
///
/// Generator returns an error.
pub(super) struct Lua;

impl Generator for Lua {
    fn generate(&mut self, _name: String) -> anyhow::Result<()> {
        Err(anyhow!("Lua generator not implemented."))
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("lua")
    }
}
