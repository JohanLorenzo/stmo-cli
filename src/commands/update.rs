#![allow(clippy::missing_errors_doc)]

use anyhow::Result;
use crate::update_checker::installed_via_cargo;

pub fn update() -> Result<()> {
    if !installed_via_cargo() {
        eprintln!("stmo-cli was not installed via `cargo install`.");
        eprintln!("To update, use the same method you used to install it.");
        anyhow::bail!("Not installed via cargo install");
    }

    let status = std::process::Command::new("cargo")
        .args(["install", "stmo-cli"])
        .status()?;

    if status.success() {
        println!("stmo-cli updated successfully.");
        Ok(())
    } else {
        anyhow::bail!("cargo install stmo-cli failed");
    }
}
