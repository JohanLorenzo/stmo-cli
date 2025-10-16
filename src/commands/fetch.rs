use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::api::RedashClient;

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

fn extract_query_ids_from_directory() -> Result<Vec<u64>> {
    let queries_dir = Path::new("queries");

    if !queries_dir.exists() {
        return Ok(Vec::new());
    }

    let mut query_ids = Vec::new();

    for entry in fs::read_dir(queries_dir).context("Failed to read queries directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "yaml")
            && let Some(filename) = path.file_name().and_then(|f| f.to_str())
            && let Some(id_str) = filename.split('-').next()
            && let Ok(id) = id_str.parse::<u64>()
        {
            query_ids.push(id);
        }
    }

    query_ids.sort_unstable();
    query_ids.dedup();

    Ok(query_ids)
}

pub async fn fetch(client: &RedashClient, query_ids: Vec<u64>, all: bool) -> Result<()> {
    fs::create_dir_all("queries")
        .context("Failed to create queries directory")?;

    let existing_query_ids = extract_query_ids_from_directory()?;

    let queries_to_fetch = if all {
        if existing_query_ids.is_empty() {
            anyhow::bail!("No queries found in queries/ directory. Use specific query IDs or run 'discover' to see available queries.");
        }
        println!("Fetching {} queries from local directory...\n", existing_query_ids.len());
        let mut queries = Vec::new();
        for id in &existing_query_ids {
            match client.get_query(*id).await {
                Ok(query) => queries.push(query),
                Err(e) => eprintln!("  ⚠ Query {id} failed to fetch: {e}"),
            }
        }
        queries
    } else if !query_ids.is_empty() {
        println!("Fetching {} specific queries...\n", query_ids.len());
        let mut queries = Vec::new();
        for id in &query_ids {
            match client.get_query(*id).await {
                Ok(query) => queries.push(query),
                Err(e) => eprintln!("  ⚠ Query {id} failed to fetch: {e}"),
            }
        }
        queries
    } else {
        anyhow::bail!("No query IDs specified. Use --all to fetch tracked queries, or provide specific query IDs.\n\nExamples:\n  cargo run -- fetch --all\n  cargo run -- fetch 123 456 789\n  cargo run -- discover  (to see available queries)");
    };

    println!("Fetching {} queries...", queries_to_fetch.len());
    for query in &queries_to_fetch {
        let slug = slugify(&query.name);
        let filename_base = format!("{}-{}", query.id, slug);

        let sql_path = format!("queries/{filename_base}.sql");
        fs::write(&sql_path, &query.sql)
            .context(format!("Failed to write {sql_path}"))?;

        let metadata = crate::models::QueryMetadata {
            id: query.id,
            name: query.name.clone(),
            description: query.description.clone(),
            data_source_id: query.data_source_id,
            user_id: query.user.as_ref().map(|u| u.id),
            schedule: query.schedule.clone(),
            options: query.options.clone(),
            visualizations: query.visualizations.clone(),
            tags: query.tags.clone(),
        };

        let yaml_path = format!("queries/{filename_base}.yaml");
        let yaml_content = serde_yaml::to_string(&metadata)
            .context("Failed to serialize query metadata")?;
        fs::write(&yaml_path, yaml_content)
            .context(format!("Failed to write {yaml_path}"))?;

        println!("  ✓ {} - {}", query.id, query.name);
    }

    println!("\n✓ All resources fetched successfully");

    Ok(())
}
