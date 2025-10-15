use anyhow::Result;
use crate::api::RedashClient;

pub async fn discover(client: &RedashClient) -> Result<()> {
    println!("Fetching your queries and dashboards from Redash...\n");

    let queries = client.fetch_all_queries().await?;
    let dashboards = client.fetch_my_dashboard_summaries().await?;

    println!("=== QUERIES ({}) ===\n", queries.len());
    for query in &queries {
        let archived = if query.is_archived { " [ARCHIVED]" } else { "" };
        let draft = if query.is_draft { " [DRAFT]" } else { "" };
        println!("  {} - {}{}{}", query.id, query.name, archived, draft);
    }

    println!("\n=== DASHBOARDS ({}) ===\n", dashboards.len());
    for dashboard in &dashboards {
        let archived = if dashboard.is_archived { " [ARCHIVED]" } else { "" };
        let draft = if dashboard.is_draft { " [DRAFT]" } else { "" };
        println!("  {} - {}{}{}", dashboard.id, dashboard.name, archived, draft);
    }

    println!("\nUse 'redash-tool init' to select resources to track.");

    Ok(())
}
