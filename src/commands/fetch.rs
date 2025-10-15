use anyhow::{Context, Result};
use std::fs;
use crate::api::RedashClient;
use crate::models::Config;

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

pub async fn fetch(client: &RedashClient) -> Result<()> {
    let config_content = fs::read_to_string("redash-config.yaml")
        .context("Failed to read redash-config.yaml. Run 'redash-tool init' first.")?;

    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse redash-config.yaml")?;

    println!("Fetching {} queries...", config.queries.len());
    for tracked in &config.queries {
        let query = client.get_query(tracked.id).await?;
        let slug = slugify(&query.name);
        let filename_base = format!("{}-{}", query.id, slug);

        let sql_path = format!("queries/{filename_base}.sql");
        fs::write(&sql_path, &query.sql)
            .context(format!("Failed to write {sql_path}"))?;

        let metadata = crate::models::QueryMetadata {
            id: query.id,
            name: query.name,
            description: query.description,
            data_source_id: query.data_source_id,
            user_id: query.user.as_ref().map(|u| u.id),
            schedule: query.schedule,
            options: query.options,
            visualizations: query.visualizations,
            tags: query.tags,
        };

        let yaml_path = format!("queries/{filename_base}.yaml");
        let yaml_content = serde_yaml::to_string(&metadata)
            .context("Failed to serialize query metadata")?;
        fs::write(&yaml_path, yaml_content)
            .context(format!("Failed to write {yaml_path}"))?;

        println!("  ✓ {} - {}", query.id, tracked.name);
    }

    println!("\nFetching {} dashboards...", config.dashboards.len());
    for tracked in &config.dashboards {
        let dashboard = client.get_dashboard(tracked.id).await?;
        let slug = slugify(&dashboard.name);
        let filename = format!("dashboards/{}-{}.yaml", dashboard.id, slug);

        let yaml_content = serde_yaml::to_string(&dashboard)
            .context("Failed to serialize dashboard")?;
        fs::write(&filename, yaml_content)
            .context(format!("Failed to write {filename}"))?;

        println!("  ✓ {} - {}", dashboard.id, tracked.name);
    }

    println!("\n✓ All resources fetched successfully");

    Ok(())
}
