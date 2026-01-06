use clap::Subcommand;

use crate::commands::{OptionalSubcommands, javascript::tauri::TauriCommands};

mod nextjs;
mod tauri;

#[derive(Subcommand)]
pub enum JSCommands {
    NextJS,
    Tauri(OptionalSubcommands<TauriCommands>),
}
