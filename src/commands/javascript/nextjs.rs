use crate::commands::{Generator, javascript::JavaScript};

/// NextJS generator
pub(super) struct NextJS;

impl Generator for NextJS {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        JavaScript.docs_path().join("nextjs")
    }
}
