use std::process::Command;

use crate::commands::{Generator, go::Go, upgrade::Upgrade, utils::execute_command};

/// Gin generator
pub(crate) struct Gin;

impl Generator for Gin {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Go.docs_path().join("gin")
    }
}

impl Upgrade for Gin {
    fn upgrade(&self) -> anyhow::Result<()> {
        execute_command(Command::new("deno").args([
            "install",
            "-A",
            "-g",
            "-f",
            "npm:@redocly/cli",
        ]))
    }
}
