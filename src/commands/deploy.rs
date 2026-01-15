use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashSet;
use crate::api::RedashClient;
use crate::models::Query;

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

fn get_all_query_metadata() -> Result<Vec<(u64, String)>> {
    let queries_dir = Path::new("queries");

    if !queries_dir.exists() {
        bail!("queries directory not found. Run 'cargo run -- fetch' first.");
    }

    let mut queries = Vec::new();

    for entry in fs::read_dir(queries_dir).context("Failed to read queries directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "yaml") {
            let metadata_content = fs::read_to_string(&path)
                .context(format!("Failed to read {}", path.display()))?;

            let metadata: crate::models::QueryMetadata = serde_yaml::from_str(&metadata_content)
                .context(format!("Failed to parse {}", path.display()))?;

            queries.push((metadata.id, metadata.name));
        }
    }

    queries.sort_by_key(|(id, _)| *id);

    Ok(queries)
}

pub async fn deploy(client: &RedashClient, query_ids: Vec<u64>, all: bool) -> Result<()> {
    let all_queries = get_all_query_metadata()?;

    let queries_to_deploy = if !query_ids.is_empty() {
        let ids_set: HashSet<_> = query_ids.iter().copied().collect();
        let filtered: Vec<_> = all_queries
            .into_iter()
            .filter(|(id, _)| ids_set.contains(id))
            .collect();

        if filtered.is_empty() {
            bail!("None of the specified query IDs were found in queries/ directory");
        }

        println!("Deploying {} specific queries...", filtered.len());
        for (id, name) in &filtered {
            println!("  → {id} - {name}");
        }
        println!();

        filtered
    } else if all {
        println!("Deploying all {} queries...\n", all_queries.len());
        all_queries
    } else {
        let changed_ids = get_changed_query_ids()?;

        if changed_ids.is_empty() {
            println!("No changed queries detected.");
            println!("Tip: Use --all to deploy all queries regardless of git status.");
            return Ok(());
        }

        let filtered: Vec<_> = all_queries
            .into_iter()
            .filter(|(id, _)| changed_ids.contains(id))
            .collect();

        println!("Deploying {} changed queries...", filtered.len());
        for (id, name) in &filtered {
            println!("  → {id} - {name}");
        }
        println!();

        filtered
    };

    for (id, name) in &queries_to_deploy {
        let slug = slugify(name);
        let sql_path = format!("queries/{id}-{slug}.sql");
        let yaml_path = format!("queries/{id}-{slug}.yaml");

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

        let result_query = if *id == 0 {
            let create_query = crate::models::CreateQuery {
                name: metadata.name,
                description: metadata.description,
                sql,
                data_source_id: metadata.data_source_id,
                schedule: metadata.schedule,
                options: Some(metadata.options),
                tags: metadata.tags,
                is_archived: false,
                is_draft: false,
            };
            let created = client.create_query(&create_query).await?;
            println!("  ✓ Created new query: {} - {name}", created.id);
            println!("    Update the YAML file with the new ID: {}", created.id);
            created
        } else {
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
            let result = client.create_or_update_query(&query).await?;
            println!("  ✓ {id} - {name}");
            result
        };

        for viz in &result_query.visualizations {
            client.update_visualization(viz).await?;
        }
    }

    println!("\n✓ All resources deployed successfully");

    Ok(())
}
