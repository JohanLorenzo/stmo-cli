use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use crate::api::RedashClient;
use crate::models::{Config, Query};

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub async fn deploy(client: &RedashClient) -> Result<()> {
    let config_content = fs::read_to_string("redash-config.yaml")
        .context("Failed to read redash-config.yaml. Run 'redash-tool init' first.")?;

    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse redash-config.yaml")?;

    println!("Deploying {} queries...", config.queries.len());
    for tracked in &config.queries {
        let slug = slugify(&tracked.name);
        let sql_path = format!("queries/{}-{}.sql", tracked.id, slug);
        let yaml_path = format!("queries/{}-{}.yaml", tracked.id, slug);

        if !Path::new(&sql_path).exists() {
            bail!("Query SQL file not found: {sql_path}");
        }
        if !Path::new(&yaml_path).exists() {
            bail!("Query metadata file not found: {yaml_path}");
        }

        let sql = fs::read_to_string(&sql_path)
            .context(format!("Failed to read {sql_path}"))?;

        let metadata_content = fs::read_to_string(&yaml_path)
            .context(format!("Failed to read {yaml_path}"))?;

        let metadata: crate::models::QueryMetadata = serde_yaml::from_str(&metadata_content)
            .context(format!("Failed to parse {yaml_path}"))?;

        let query = Query {
            id: metadata.id,
            name: metadata.name,
            description: metadata.description,
            sql,
            data_source_id: metadata.data_source_id,
            user: None,
            schedule: metadata.schedule,
            options: metadata.options,
            visualizations: metadata.visualizations,
            tags: metadata.tags,
            is_archived: false,
            is_draft: false,
            updated_at: String::new(),
            created_at: String::new(),
        };

        client.create_or_update_query(&query).await?;
        println!("  ✓ {} - {}", tracked.id, tracked.name);
    }

    if !config.dashboards.is_empty() {
        println!("\nDashboard deployment is not currently supported.");
        println!("  The Redash API does not provide a reliable way to update dashboards programmatically.");
        println!("  Dashboards can be fetched and version-controlled, but must be updated via the Redash UI.");
        println!("  {} dashboards skipped", config.dashboards.len());
    }

    println!("\n✓ All resources deployed successfully");

    Ok(())
}
