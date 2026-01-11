use std::{fs, path::PathBuf};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::Context;
use clap::Subcommand;
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DEFAULT_CONFIG_DIR, DOCS_DIR,
    commands::{
        Commands, Generator,
        cpp::Cpp,
        go::{Go, GoCommands},
        javascript::{JSCommands, JavaScript},
        kotlin::{KTCommands, Kotlin},
        python::Python,
        rust::Rust,
        utils::OptionalSubcommands,
    },
    config_dir,
};

#[derive(Subcommand)]
pub enum RootCommands {
    /// Generate C++ projects.
    Cpp,

    /// Generate Go projects.
    Go(OptionalSubcommands<GoCommands>),

    /// Generate JavaScript projects.
    #[command(alias = "js")]
    JavaScript(OptionalSubcommands<JSCommands>),

    /// Generate Kotlin projects.
    #[command(alias = "kt")]
    Kotlin(OptionalSubcommands<KTCommands>),

    /// Generate Python projects.
    #[command(alias = "py")]
    Python,

    /// Generate Rust projects.
    #[command(alias = "rs")]
    Rust,
}

impl Commands for RootCommands {
    fn generator(self) -> Box<dyn Generator> {
        match self {
            RootCommands::Cpp => Box::new(Cpp),
            RootCommands::Go(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(Go),
            },
            RootCommands::JavaScript(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(JavaScript),
            },
            RootCommands::Kotlin(optional_subcommands) => match optional_subcommands.command {
                Some(c) => c.generator(),
                None => Box::new(Kotlin),
            },
            RootCommands::Python => Box::new(Python),
            RootCommands::Rust => Box::new(Rust),
        }
    }
}

/// Root generator
///
/// Generator returns an error.
#[derive(Debug)]
pub struct Root;

impl Generator for Root {
    #[instrument]
    fn generate(&self, _name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(4);

        info!("Generating config directory.");

        let config_dir = config_dir();

        println!(
            "{msg_style}Generating config directory in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            config_dir.display()
        );
        info!("Creating config directory");
        info!(path = config_dir.to_str());

        fs::create_dir_all(&config_dir).context("Failed to create config directory.")?;
        bar.inc(1);

        info!("Done");
        info!("Generating config files");

        DEFAULT_CONFIG_DIR
            .extract(&config_dir)
            .context("Failed to generate config files.")?;

        bar.inc(1);

        info!("Done");
        info!("Creating documentation directory");

        let docs_dir = config_dir.join("docs");

        info!(path = docs_dir.to_str());

        fs::create_dir_all(&docs_dir)?;
        bar.inc(1);

        info!("Done");
        info!("Generating documentation files.");

        DOCS_DIR
            .extract(&docs_dir)
            .context("Failed to generate documentation files")?;

        bar.inc(1);

        info!("Done");
        info!("Done generating config directory.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> PathBuf {
        PathBuf::from(".")
    }
}
