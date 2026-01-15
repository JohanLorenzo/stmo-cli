mod api;
mod commands;
mod models;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use api::RedashClient;

#[derive(Parser)]
#[command(name = "redash-tool")]
#[command(about = "Version control tool for Redash queries", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List all queries from Redash")]
    Discover,

    #[command(about = "Create queries directory")]
    Init,

    #[command(about = "Fetch queries from Redash")]
    Fetch {
        #[arg(help = "Query IDs to fetch (e.g., 123 456 789)")]
        query_ids: Vec<u64>,
        #[arg(long, help = "Fetch all queries currently tracked in queries/ directory")]
        all: bool,
    },

    #[command(about = "Deploy local changes to Redash (only changed queries by default)")]
    Deploy {
        #[arg(help = "Query IDs to deploy (e.g., 123 456 789)")]
        query_ids: Vec<u64>,
        #[arg(long, help = "Deploy all queries instead of only changed ones")]
        all: bool,
    },
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
        Commands::Init => commands::init::init()?,
        Commands::Fetch { query_ids, all } => commands::fetch::fetch(&client, query_ids, all).await?,
        Commands::Deploy { query_ids, all } => commands::deploy::deploy(&client, query_ids, all).await?,
    }

    Ok(())
}
