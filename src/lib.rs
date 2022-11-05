#![warn(clippy::all, clippy::cargo, clippy::pedantic, clippy::nursery)]

use console::style;
/// # Errors
/// Errors when cargo metadata fails
pub fn run() -> Result<(), Error> {
    let meta = cargo_metadata::MetadataCommand::new().exec()?;
    let exe_meta = meta.root_package().ok_or(Error::ExpectedPackage)?;
    println!("{} x86_64", style("Building").green().bold());
    std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("x86_64-apple-darwin")
        .spawn()?
        .wait()?;
    println!("{} aarch64", style("Building").green().bold());
    std::process::Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--target")
        .arg("aarch64-apple-darwin")
        .spawn()?
        .wait()?;
    println!("{} {}", style("Linking").green().bold(), &exe_meta.name);
    let root_path = meta.target_directory.canonicalize()?;
    let mut output = root_path.join("universal2-apple-darwin").join("release");
    std::fs::create_dir_all(&output)?;
    output.push(&exe_meta.name);
    let x86_bin = root_path
        .join("x86_64-apple-darwin")
        .join("release")
        .join(&exe_meta.name);
    let arm_bin = root_path
        .join("aarch64-apple-darwin")
        .join("release")
        .join(&exe_meta.name);
    std::process::Command::new("lipo")
        .arg("-create")
        .arg("-output")
        .arg(output)
        .arg(x86_bin)
        .arg(arm_bin)
        .spawn()?
        .wait()?;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("cargo_metadata error: {0}")]
    CargoMetadata(#[from] cargo_metadata::Error),
    #[error("Expected at least one package!")]
    ExpectedPackage,
}
