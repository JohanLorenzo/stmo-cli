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

    #[command(about = "Execute a query and display results")]
    Execute {
        #[arg(help = "Query ID to execute (must be fetched locally first)")]
        query_id: u64,

        #[arg(long, help = "Query parameter in format: name=value (can be used multiple times)")]
        param: Vec<String>,

        #[arg(long, short = 'f', default_value = "json", help = "Output format: json or table")]
        format: String,

        #[arg(long, short = 'i', help = "Prompt for missing parameters interactively")]
        interactive: bool,

        #[arg(long, default_value = "300", help = "Timeout in seconds")]
        timeout: u64,

        #[arg(long, help = "Limit number of rows displayed (default: 100)")]
        limit: Option<usize>,
    },

    #[command(about = "List and explore data sources")]
    DataSources {
        #[arg(help = "Optional: Data source ID to inspect")]
        data_source_id: Option<u64>,

        #[arg(long, help = "Show table schema for the data source")]
        schema: bool,

        #[arg(long, help = "Force refresh schema from data source (slower but always up-to-date)")]
        refresh: bool,

        #[arg(long, short = 'f', default_value = "json", help = "Output format: json or table")]
        format: String,
    },

    #[command(about = "Archive queries in Redash and remove local files")]
    Archive {
        #[arg(help = "Query IDs to archive (e.g., 123 456 789)")]
        query_ids: Vec<u64>,

        #[arg(long, help = "Remove local files for queries already archived in Redash")]
        cleanup: bool,
    },

    #[command(about = "Restore archived queries")]
    Unarchive {
        #[arg(help = "Query IDs to unarchive (e.g., 123 456 789)")]
        query_ids: Vec<u64>,
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
        Commands::Execute { query_id, param, format, interactive, timeout, limit } => {
            let output_format = format.parse::<commands::OutputFormat>()
                .context("Invalid output format")?;
            let limit_rows = limit.or(Some(100));
            commands::execute::execute(
                &client,
                query_id,
                param,
                output_format,
                interactive,
                timeout,
                limit_rows,
            ).await?;
        }
        Commands::DataSources { data_source_id, schema, refresh, format } => {
            let output_format = format.parse::<commands::OutputFormat>()
                .context("Invalid output format")?;

            if let Some(id) = data_source_id {
                commands::datasources::show_data_source(&client, id, schema, refresh, output_format).await?;
            } else {
                commands::datasources::list_data_sources(&client, output_format).await?;
            }
        }
        Commands::Archive { query_ids, cleanup } => {
            if cleanup {
                commands::archive::cleanup(&client).await?;
            } else if !query_ids.is_empty() {
                commands::archive::archive(&client, query_ids).await?;
            } else {
                anyhow::bail!("No query IDs specified. Use specific query IDs or --cleanup flag.\n\nExamples:\n  cargo run -- archive 123 456\n  cargo run -- archive --cleanup");
            }
        }
        Commands::Unarchive { query_ids } => {
            if query_ids.is_empty() {
                anyhow::bail!("No query IDs specified. Provide query IDs to unarchive.\n\nExample:\n  cargo run -- unarchive 123 456");
            }
            commands::archive::unarchive(&client, query_ids).await?;
        }
    }

    Ok(())
}
