use std::process::Command;

use crate::commands::{Generator, root::Root, upgrade::Upgrade, utils::execute_command};

/// Python generator
pub(super) struct Python;

impl Generator for Python {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("python")
    }
}

impl Upgrade for Python {
    fn upgrade(&self) -> anyhow::Result<()> {
        execute_command(Command::new("uv").args(["tool", "upgrade", "--all"]))?;
        execute_command(Command::new("uv").args(["cache", "prune"]))
    }
}
