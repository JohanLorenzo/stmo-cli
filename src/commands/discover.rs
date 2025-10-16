use anyhow::Result;
use crate::api::RedashClient;

pub async fn discover(client: &RedashClient) -> Result<()> {
    println!("Fetching your queries from Redash...\n");

    let queries = client.fetch_all_queries().await?;

    println!("=== QUERIES ({}) ===\n", queries.len());
    for query in &queries {
        let archived = if query.is_archived { " [ARCHIVED]" } else { "" };
        let draft = if query.is_draft { " [DRAFT]" } else { "" };
        println!("  {} - {}{}{}", query.id, query.name, archived, draft);
    }

    println!("\nUse 'redash-tool init' to create the queries directory.");

    Ok(())
}
