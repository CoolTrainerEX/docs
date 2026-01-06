use anyhow::Result;
use clap::Parser;
use docs::{OptionalSubcommands, commands::RootCommands};

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Open documentation.
    #[arg(short, long, global = true)]
    docs: bool,
    /// Project name.
    #[arg(short, long, global = true)]
    name: Option<String>,
    #[command(flatten)]
    command: OptionalSubcommands<RootCommands>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    Ok(())
}
