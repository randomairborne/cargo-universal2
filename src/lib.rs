#![warn(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]
pub fn run() -> Result<(), Error> {
    cargo_metadata::MetadataCommand::new().exec()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cargo_metadata encountered an error: {0}")]
    CargoMetadata(#[from] cargo_metadata::Error),
}
