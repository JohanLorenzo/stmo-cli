pub mod discover;
pub mod init;
pub mod fetch;
pub mod deploy;
pub mod execute;
pub mod datasources;
pub mod archive;

use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Table,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "table" => Ok(Self::Table),
            _ => bail!("Invalid format. Use: json or table"),
        }
    }
}
