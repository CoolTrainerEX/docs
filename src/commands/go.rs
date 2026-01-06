use clap::Subcommand;

mod gin;

#[derive(Subcommand)]
pub enum GoCommands {
    Gin,
}
