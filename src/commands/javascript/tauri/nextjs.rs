use crate::{Generator, commands::javascript::tauri::Tauri};

#[derive(Default)]
pub struct NextJS {}

impl Generator for NextJS {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Tauri::default().docs_path().join("nextjs")
    }
}
