use std::process::Command;

use crate::commands::{Generator, root::Root, upgrade::Upgrade, utils::execute_command};

/// C++ generator
pub(super) struct Cpp;

impl Generator for Cpp {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root.docs_path().join("cpp")
    }
}

impl Upgrade for Cpp {
    fn upgrade(&self) -> anyhow::Result<()> {
        execute_command(Command::new("conan").args(["cache", "clean"]))
    }
}
