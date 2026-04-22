use std::{env, fs, process::Command};

use anstream::println;
use anstyle::{AnsiColor, Style};
use anyhow::{Context, anyhow};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use indicatif::ProgressBar;
use tracing::{info, instrument};

use crate::{
    DOCS_DIR,
    commands::{
        Generator,
        root::Root,
        upgrade::Upgrade,
        utils::{Deps, execute_command, extract_files},
    },
};

/// C++ generator
#[derive(Debug)]
pub(super) struct Cpp;

impl Generator for Cpp {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));
        let strong_style = Style::new().bold();
        let bar = ProgressBar::new(4);

        info!("Generating C++ project.");

        let current_dir = env::current_dir().context("Failed to get current directory")?;
        let proj_dir = current_dir.join(&name);

        println!(
            "{msg_style}Generating C++ project in {msg_style:#}{strong_style}{}{strong_style:#}{msg_style}.{msg_style:#}",
            proj_dir.display()
        );
        info!("Running init command.");

        fs::create_dir_all(&proj_dir).context("Failed to create directory.")?;
        env::set_current_dir(&proj_dir).context("Failed to change working directory.")?;

        info!(dir = proj_dir.to_str());

        let mut command = Command::new("conan");
        let type_choices = ["lib", "exe"];

        let deps: Deps = serde_json::from_slice(
            DOCS_DIR
                .get_file(self.docs_path().join("deps").with_extension("json"))
                .context(format!(
                    "Cannot find {strong_style}deps.json{strong_style:#}."
                ))?
                .contents(),
        )
        .context(format!(
            "Failed to parse {strong_style}deps.json{strong_style:#}."
        ))?;

        command.args([
            "new",
            match type_choices[Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Project type")
                .items(type_choices)
                .interact()
                .context("Input error.")?]
            {
                "lib" => "cmake_lib",
                "exe" => "cmake_exe",
                _ => return Err(anyhow!("Invalid choice.")),
            },
            "-d",
            &format!("name={}", &name),
        ]);

        if let Some(deps) = deps.deps {
            command.args(deps.iter().flat_map(|dep| {
                [
                    String::from("-d"),
                    format!(
                        "requires={}/{}",
                        dep,
                        Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(format!("{} version", dep))
                            .default(String::from("[*]"))
                            .interact_text()
                            .context("Input error.")
                            .unwrap_or(String::from("[*]"))
                    ),
                ]
            }));
        }

        if let Some(dev) = deps.dev {
            command.args(dev.iter().flat_map(|dep| {
                [
                    String::from("-d"),
                    format!(
                        "tool_requires={}/{}",
                        dep,
                        Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(format!("{} version", dep))
                            .default(String::from("[*]"))
                            .interact_text()
                            .context("Input error.")
                            .unwrap_or(String::from("[*]"))
                    ),
                ]
            }));
        }

        execute_command(&mut command)?;
        bar.inc(1);

        info!("Done.");

        info!("Installing dependencies");

        execute_command(Command::new("conan").args(["install", ".", "--build=missing"]))?;
        execute_command(Command::new("conan").args([
            "install",
            ".",
            "-s",
            "build_type=Debug",
            "--build=missing",
        ]))?;

        bar.inc(1);

        info!("Done.");
        info!("Running documentation init command.");

        execute_command(Command::new("doxygen").arg("-g"))?;
        bar.inc(1);

        info!("Done.");
        info!("Creating files.");

        extract_files(
            DOCS_DIR
                .get_dir(self.docs_path().join("create"))
                .context("Cannot find create directory.")?,
            &proj_dir,
        )?;

        bar.inc(1);

        info!("Done.");
        info!("Done generating project.");

        bar.finish_and_clear();

        Ok(())
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("cpp")
    }
}

impl Upgrade for Cpp {
    #[instrument]
    fn upgrade(&self) -> anyhow::Result<()> {
        let msg_style = Style::new().fg_color(Some(AnsiColor::Blue.into()));

        info!("Upgrading C++.");
        println!("{msg_style}Upgrading C++.{msg_style:#}");
        info!("Upgrading apps.");

        execute_command(Command::new("apt").arg("update"))?;
        execute_command(Command::new("apt").arg("upgrade"))?;

        info!("Done.");
        info!("Clearing cache.");

        execute_command(Command::new("apt").arg("clean"))?;
        execute_command(Command::new("apt").arg("autoclean"))?;
        execute_command(Command::new("apt").arg("autoremove"))?;
        execute_command(Command::new("conan").args(["cache", "clean"]))?;

        info!("Done.");
        info!("Done upgrading C++.");

        Ok(())
    }
}
