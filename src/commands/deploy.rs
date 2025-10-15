use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashSet;
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

fn get_changed_query_ids() -> Result<HashSet<u64>> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .context("Failed to run git status. Make sure you're in a git repository.")?;

    if !output.status.success() {
        bail!("git status command failed");
    }

    let stdout = String::from_utf8(output.stdout)
        .context("Failed to parse git status output")?;

    let mut changed_ids = HashSet::new();

    for line in stdout.lines() {
        if line.len() < 3 {
            continue;
        }

        let file_path = &line[3..];
        let path = Path::new(file_path);

        if file_path.starts_with("queries/")
            && path.extension().is_some_and(|ext| {
                ext.eq_ignore_ascii_case("sql") || ext.eq_ignore_ascii_case("yaml")
            })
            && let Some(filename) = file_path.strip_prefix("queries/")
            && let Some(id_str) = filename.split('-').next()
            && let Ok(id) = id_str.parse::<u64>()
        {
            changed_ids.insert(id);
        }
    }

    Ok(changed_ids)
}

pub async fn deploy(client: &RedashClient, all: bool) -> Result<()> {
    let config_content = fs::read_to_string("redash-config.yaml")
        .context("Failed to read redash-config.yaml. Run 'redash-tool init' first.")?;

    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse redash-config.yaml")?;

    let queries_to_deploy = if all {
        println!("Deploying all {} queries...", config.queries.len());
        config.queries.clone()
    } else {
        let changed_ids = get_changed_query_ids()?;

        if changed_ids.is_empty() {
            println!("No changed queries detected.");
            println!("Tip: Use --all to deploy all queries regardless of git status.");
            return Ok(());
        }

        let filtered: Vec<_> = config.queries.iter()
            .filter(|q| changed_ids.contains(&q.id))
            .cloned()
            .collect();

        println!("Deploying {} changed queries...", filtered.len());
        for id in &changed_ids {
            if let Some(q) = config.queries.iter().find(|q| q.id == *id) {
                println!("  → {} - {}", id, q.name);
            }
        }
        println!();

        filtered
    };

    for tracked in &queries_to_deploy {
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
