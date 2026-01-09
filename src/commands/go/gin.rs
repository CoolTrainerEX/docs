use crate::{Generator, commands::go::Go};

#[derive(Default)]
pub struct Gin {}

impl Generator for Gin {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }
    
    fn docs_path(&self) -> std::path::PathBuf {
        Go::default().docs_path().join("gin")
    }

}
