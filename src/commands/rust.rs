use crate::{Generator, commands::Root};

#[derive(Default)]
pub struct Rust {}

impl Generator for Rust {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("rust")
    }
}
