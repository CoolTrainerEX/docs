use crate::{Generator, commands::Root};

/// C++ generator
#[derive(Default)]
pub(super) struct Cpp;

impl Generator for Cpp {
    fn generate(&self, name: String) -> anyhow::Result<()> {
        todo!()
    }

    fn docs_path(&self) -> std::path::PathBuf {
        Root::default().docs_path().join("cpp")
    }
}
