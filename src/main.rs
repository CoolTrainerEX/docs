use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use dialoguer::{Input, theme::ColorfulTheme};
use docs::{
    Commands, OptionalSubcommands,
    commands::{Root, RootCommands},
};
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    path: PathBuf,
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Open documentation.
    #[arg(short, long, global = true)]
    docs: bool,
    /// Project name.
    #[arg(short, long, global = true)]
    name: Option<String>,
    /// Config file path
    #[arg(short, long, global = true, default_value = "config.toml")]
    config: PathBuf,
    #[command(flatten)]
    command: OptionalSubcommands<RootCommands>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config: Config = Figment::new()
        .merge(Toml::file_exact(args.config))
        .merge(Env::prefixed("DOCS_"))
        .extract()?;

    let generator = match args.command.command {
        Some(c) => c.generator(),
        None => Box::new(Root::default()),
    };

    if args.docs {
        let mut path = config.path.join(generator.docs_path());

        path.push("README");
        path.set_extension("md");

        open::that(path)?;
    } else {
        generator.generate(match args.name {
            Some(name) => name,
            None => Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Project name")
                .interact_text()?,
        })?;
    }

    Ok(())
}
