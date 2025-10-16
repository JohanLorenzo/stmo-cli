use anyhow::{Context, Result};
use std::fs;

pub fn init() -> Result<()> {
    fs::create_dir_all("queries")
        .context("Failed to create queries directory")?;
    fs::create_dir_all("dashboards")
        .context("Failed to create dashboards directory")?;

    println!("✓ Created queries/ and dashboards/ directories");
    println!("\nNext steps:");
    println!("  1. Run 'cargo run -- fetch' to download all your queries from Redash");
    println!("  2. Run 'cargo run -- deploy' to push local changes back to Redash");
    println!("  3. Use 'cargo run -- discover' to see all available queries");

    Ok(())
}
