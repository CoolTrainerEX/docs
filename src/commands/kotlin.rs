use clap::Subcommand;

mod fabric;

#[derive(Subcommand)]
pub enum KTCommands {
    Fabric,
}
