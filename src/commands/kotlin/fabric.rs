use crate::{Generator, commands::kotlin::Kotlin};

#[derive(Default)]
pub struct Fabric {}

impl Generator for Fabric {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Kotlin::default().docs_path().join("fabric")
    }
}
