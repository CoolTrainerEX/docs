use std::{env, ffi::OsStr, iter, path::PathBuf, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::Context;
use indicatif::ProgressBar;
use serde::Deserialize;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{Generator, root::Root, utils::execute_command},
};

/// Rust generator
#[derive(Debug)]
pub(super) struct Rust;

impl Generator for Rust {
    #[instrument]
    fn generate(&self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(2);

        info!("Generating Rust project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating Rust project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");
        info!(dir = current_dir.to_str());

        execute_command(Command::new("cargo").args(["new", &name]))?;
        bar.inc(1);

        info!("Done");
        info!("Installing dependencies");

        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        let deps: RustDeps = serde_json::from_slice(
            DOCS_DIR
                .get_file(PathBuf::from("rust").join("deps").with_extension("json"))
                .context(format!(
                    "Cannot find {strong_style}deps.json{strong_style:#}."
                ))?
                .contents(),
        )
        .context(format!(
            "Failed to parse {strong_style}deps.json{strong_style:#}."
        ))?;

        execute_command(
            Command::new("cargo")
                .arg("add")
                .args(parse_deps(&deps.deps)),
        )?;

        execute_command(
            Command::new("cargo")
                .arg("add")
                .args(parse_deps(&deps.dev))
                .arg("--dev"),
        )?;

        bar.inc(1);

        info!("Done");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("rust")
    }
}

/// Entry in [`RustDeps`]
#[derive(Deserialize)]
#[serde(untagged)]
enum DepEntry {
    Dep(String),
    WithFeatures { name: String, features: Vec<String> },
}

/// `deps.json` structure in documentation.
#[derive(Deserialize)]
struct RustDeps {
    /// List of dependencies.
    deps: Vec<DepEntry>,

    /// List of dev dependencies.
    dev: Vec<DepEntry>,
}

fn parse_deps(deps: &[DepEntry]) -> impl IntoIterator<Item = impl AsRef<OsStr>> {
    deps.iter().flat_map(|dep| match dep {
        DepEntry::Dep(name) => vec![name.to_owned()],
        DepEntry::WithFeatures { name, features } => iter::once(name.to_owned())
            .chain(
                features
                    .iter()
                    .flat_map(|feature| [String::from("--features"), format!("{name}/{feature}")]),
            )
            .collect(),
    })
}
