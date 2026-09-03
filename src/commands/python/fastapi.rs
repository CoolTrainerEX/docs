use tracing::instrument;

use crate::commands::{Generator, python::Python};

/// NextJS generator
#[derive(Debug)]
pub(super) struct FastAPI;

impl Generator for FastAPI {
    #[instrument]
    fn generate(&mut self, name: String) -> anyhow::Result<()> {
        todo!();
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Python.docs_path().join("fastapi")
    }
}
