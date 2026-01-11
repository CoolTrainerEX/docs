use std::process::Command;

use crate::commands::{Generator, kotlin::Kotlin, upgrade::Upgrade, utils::execute_command};

/// Fabric generator
pub(crate) struct Fabric;

impl Generator for Fabric {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Kotlin.docs_path().join("fabric")
    }
}

impl Upgrade for Fabric {
    fn upgrade(&self) -> anyhow::Result<()> {
        execute_command(Command::new("fabric").arg("upgrade"))
    }
}
