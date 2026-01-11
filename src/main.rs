use std::path::PathBuf;

use anstyle::Style;
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Input, theme::ColorfulTheme};
use docs::{
    commands::{Commands, Subcommands, root::Root, upgrade::upgrade, utils::OptionalSubcommands},
    config_dir, config_dir_exists_or_gen, default_config,
};
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use serde::Deserialize;
use validator::Validate;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Open documentation.
    #[arg(short, long, global = true)]
    docs: bool,

    /// Open local documentation.
    #[arg(short, long, global = true)]
    local: bool,

    /// Config file path
    #[arg(short, long, global = true, default_value_os_t = default_config())]
    config: PathBuf,

    /// Project name.
    name: Option<String>,
    #[command(flatten)]
    command: OptionalSubcommands<Subcommands>,
}

#[derive(Deserialize, Validate)]
struct Config {
    /// Path to the documentation.
    path: Option<PathBuf>,

    /// URL to the documentation.
    #[validate(url)]
    url: Option<String>,
}

fn main() -> Result<()> {
    let strong_style = Style::new().bold();

    let (non_blocking, _guard) = tracing_appender::non_blocking(tracing_appender::rolling::hourly(
        config_dir().join("log"),
        "docs.log",
    ));

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .init();

    let args = Args::parse();

    config_dir_exists_or_gen()?;

    let config: Config = Figment::new()
        .merge(Toml::file_exact(args.config))
        .merge(Env::prefixed("DOCS_"))
        .extract()
        .context("Failed to open config file.")?;

    config.validate().context("Invalid config values.")?;

    let mut is_root = false;

    let generator = match args.command.command {
        Some(s) => match s {
            Subcommands::Generate(root_commands) => root_commands.generator(),
            Subcommands::Upgrade => {
                return upgrade();
            }
        },
        None => {
            is_root = true;
            Box::new(Root)
        }
    };

    if args.docs {
        open::that(
            PathBuf::from(config.url.context(format!(
                "Config missing {strong_style}url{strong_style:#} value"
            ))?)
            .join(generator.docs_path()),
        )
        .context("Failed to open documentation.")
    } else if args.local {
        open::that(
            PathBuf::from(config.path.context(format!(
                "Config missing {strong_style}path{strong_style:#} value"
            ))?)
            .join(generator.docs_path())
            .join("README")
            .with_extension("md"),
        )
        .context("Failed to open local documentation.")
    } else {
        generator.generate(if is_root {
            String::new()
        } else {
            match args.name {
                Some(name) => name,
                None => Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Project name")
                    .interact_text()
                    .context("Input error.")?,
            }
        })
    }
}
