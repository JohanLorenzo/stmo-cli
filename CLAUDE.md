# redash-tool

Rust CLI for version controlling Redash queries and dashboards.

## Quick Reference

**Commands**: `discover` `init` `fetch` `deploy`
**Env Vars**: `REDASH_API_KEY` (required), `REDASH_URL` (optional, defaults to sql.telemetry.mozilla.org)

## Key Constraints

- All clippy::pedantic warnings = errors (pre-commit enforced)
- No docstrings (user preference)
- Borrowed strings preferred (`&str` vs `String`)
- Break complex logic into well-named functions
- Error handling via `anyhow`
- No redundant comments (clear naming instead)

## Project Structure

```
src/
├── main.rs              # CLI entry point with clap
├── api.rs               # Redash API client
├── models.rs            # Data structures
└── commands/
    ├── mod.rs           # OutputFormat enum
    ├── discover.rs      # List all resources
    ├── init.rs          # Create directory
    ├── fetch.rs         # Download queries, slugify()
    └── deploy.rs        # Upload changes
```

## Data Models

**Query**: Full Redash query (id, name, sql, data_source_id, options.parameters, visualizations, schedule, user)
**QueryMetadata**: YAML variant (excludes user, uses user_id)
**Dashboard**: Full dashboard with widgets
**DashboardSummary**: Lightweight dashboard listing

## API Client (api.rs)

**Query**: list_my_queries, get_query, fetch_all_queries, create_or_update_query
**Dashboard**: get_dashboard, fetch_my_dashboard_summaries, create_or_update_dashboard

## Testing Guidelines

### Test Isolation
Tests must NEVER touch production directories (`queries/`, `dashboards/`). Use `tempfile::TempDir`:

```rust
use tempfile::TempDir;

#[test]
fn test_something() {
    let temp_dir = TempDir::new().unwrap();
    // Use temp_dir.path() for all file operations
}
```

### API Error Handling
Don't use `.error_for_status()` - it discards the response body. Instead:

```rust
let status = response.status();
if !status.is_success() {
    let error_body = response.text().await.unwrap_or_default();
    anyhow::bail!("API error {status}: {error_body}");
}
```

## Redash API Development

**IMPORTANT: Always verify before planning**

1. **Test endpoints exist** - Don't assume an endpoint exists because a similar one does. Test with curl:
   ```bash
   curl -s -w "%{http_code}" "https://sql.telemetry.mozilla.org/api/<endpoint>" \
     -H "Authorization: Key ${REDASH_API_KEY}"
   ```

2. **Check actual response fields** - The models may not match what the API returns. Inspect raw JSON:
   ```bash
   curl -s "https://sql.telemetry.mozilla.org/api/<endpoint>?page=1&page_size=1" \
     -H "Authorization: Key ${REDASH_API_KEY}" | jq '.results[0]'
   ```

3. **Verify filter/mutation support** - A field in responses doesn't mean you can filter by it or set it via API. Test POST/query params explicitly.

4. **STMO may differ from upstream** - Mozilla's instance may have endpoints disabled or behave differently than Redash documentation suggests.
