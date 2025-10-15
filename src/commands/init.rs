use anyhow::{Context, Result};
use dialoguer::MultiSelect;
use std::fs;
use crate::api::RedashClient;
use crate::models::{Config, TrackedResource};

pub async fn init(client: &RedashClient) -> Result<()> {
    println!("Fetching your queries and dashboards from Redash...\n");

    let queries = client.fetch_all_queries().await?;
    let dashboards = client.fetch_my_dashboard_summaries().await?;

    let query_items: Vec<String> = queries
        .iter()
        .map(|q| {
            let archived = if q.is_archived { " [ARCHIVED]" } else { "" };
            let draft = if q.is_draft { " [DRAFT]" } else { "" };
            format!("{} - {}{}{}", q.id, q.name, archived, draft)
        })
        .collect();

    let dashboard_items: Vec<String> = dashboards
        .iter()
        .map(|d| {
            let archived = if d.is_archived { " [ARCHIVED]" } else { "" };
            let draft = if d.is_draft { " [DRAFT]" } else { "" };
            format!("{} - {}{}{}", d.id, d.name, archived, draft)
        })
        .collect();

    println!("Select queries to track (space to select, enter to confirm):");
    let selected_query_indices = MultiSelect::new()
        .items(&query_items)
        .interact()
        .context("Failed to get query selection")?;

    println!("\nSelect dashboards to track (space to select, enter to confirm):");
    let selected_dashboard_indices = MultiSelect::new()
        .items(&dashboard_items)
        .interact()
        .context("Failed to get dashboard selection")?;

    let tracked_queries: Vec<TrackedResource> = selected_query_indices
        .iter()
        .map(|&i| TrackedResource {
            id: queries[i].id,
            name: queries[i].name.clone(),
            slug: None,
        })
        .collect();

    let tracked_dashboards: Vec<TrackedResource> = selected_dashboard_indices
        .iter()
        .map(|&i| TrackedResource {
            id: dashboards[i].id,
            name: dashboards[i].name.clone(),
            slug: Some(dashboards[i].slug.clone()),
        })
        .collect();

    let config = Config {
        queries: tracked_queries,
        dashboards: tracked_dashboards,
    };

    let config_yaml = serde_yaml::to_string(&config)
        .context("Failed to serialize config")?;

    fs::write("redash-config.yaml", config_yaml)
        .context("Failed to write config file")?;

    fs::create_dir_all("queries")
        .context("Failed to create queries directory")?;
    fs::create_dir_all("dashboards")
        .context("Failed to create dashboards directory")?;

    println!("\n✓ Configuration saved to redash-config.yaml");
    println!("  {} queries selected", config.queries.len());
    println!("  {} dashboards selected", config.dashboards.len());
    println!("\nRun 'redash-tool fetch' to download the selected resources.");

    Ok(())
}
