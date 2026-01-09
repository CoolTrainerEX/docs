use crate::{Generator, commands::Root};

#[derive(Default)]
pub struct Python {}

impl Generator for Python {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("python")
    }
}
