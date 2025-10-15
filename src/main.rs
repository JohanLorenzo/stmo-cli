mod api;
mod commands;
mod models;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use api::RedashClient;

#[derive(Parser)]
#[command(name = "redash-tool")]
#[command(about = "Version control tool for Redash queries and dashboards", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List all queries and dashboards from Redash")]
    Discover,

    #[command(about = "Interactively select queries and dashboards to track")]
    Init,

    #[command(about = "Fetch tracked queries and dashboards from Redash")]
    Fetch,

    #[command(about = "Deploy local changes to Redash")]
    Deploy,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let api_key = std::env::var("REDASH_API_KEY")
        .context("REDASH_API_KEY environment variable not set")?;

    let base_url = std::env::var("REDASH_URL")
        .unwrap_or_else(|_| "https://sql.telemetry.mozilla.org".to_string());

    let client = RedashClient::new(base_url, &api_key)?;

    match cli.command {
        Commands::Discover => commands::discover::discover(&client).await?,
        Commands::Init => commands::init::init(&client).await?,
        Commands::Fetch => commands::fetch::fetch(&client).await?,
        Commands::Deploy => commands::deploy::deploy(&client).await?,
    }

    Ok(())
}
