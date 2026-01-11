use crate::commands::{Generator, javascript::JavaScript};

/// NextJS generator
#[derive(Default)]
pub(super) struct NextJS;

impl Generator for NextJS {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        JavaScript::default().docs_path().join("nextjs")
    }
}
