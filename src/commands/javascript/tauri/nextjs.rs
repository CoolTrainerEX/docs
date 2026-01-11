use crate::commands::{Generator, javascript::tauri::Tauri};

/// Tauri NextJS generator
pub(super) struct NextJS;

impl Generator for NextJS {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Tauri.docs_path().join("nextjs")
    }
}
