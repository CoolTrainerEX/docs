use anyhow::Result;
use clap::Parser;
use dialoguer::Input;
use docs::{
    Commands, OptionalSubcommands,
    commands::{Root, RootCommands},
};

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

    let generator = match args.command.command {
        Some(c) => c.generator(),
        None => Box::new(Root::default()),
    };

    if args.docs {
        generator.docs_path();
    } else {
        generator.generate(match args.name {
            Some(name) => name,
            None => Input::new()
                .with_prompt("Project name")
                .interact_text()
                .unwrap(),
        })?;
    }

    Ok(())
}
