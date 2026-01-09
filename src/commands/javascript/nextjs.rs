use crate::{Generator, commands::javascript::Javascript};

#[derive(Default)]
pub struct NextJS {}

impl Generator for NextJS {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Javascript::default().docs_path().join("nextjs")
    }
}
