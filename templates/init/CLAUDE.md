# Redash Version Control Repository

This repository contains version-controlled Redash queries and dashboards managed by `stmo-cli`.

## Quick Reference

**Commands**: `discover` `fetch` `deploy` `execute` `data-sources` `archive` `unarchive` `dashboards`
**File Naming**: `queries/{id}-{slug}.sql` + `queries/{id}-{slug}.yaml`, `dashboards/{id}-{slug}.yaml`
**Env Vars**: `REDASH_API_KEY` (required), `REDASH_URL` (optional, defaults to sql.telemetry.mozilla.org)

## Data Exploration (AI Assistants)

**IMPORTANT**: Clean up after exploration. Archive any queries you fetch.

1. **Find data sources**: `stmo-cli data-sources`
2. **Explore schema**: `stmo-cli data-sources <id> --schema`
3. **Discover queries**: `stmo-cli discover`
4. **Fetch query**: `stmo-cli fetch <id>` → read `queries/<id>-*.sql`
5. **Execute**: `stmo-cli execute <id> --format table --limit 50`
6. **Clean up**: `stmo-cli archive <id>` (MANDATORY)

To restore: `stmo-cli unarchive <id> && stmo-cli fetch <id>`

## Commands

### Queries
**discover**: List all queries (IDs + names)
**fetch**: Download queries (`--all` for tracked, or `<ids>`)
**deploy**: Upload changes (`--all` for everything)
**execute**: Run query (`--param key=val`, `--format table|json`, `--interactive`)
**data-sources**: List sources, `<id> --schema` for tables
**archive**: Archive queries + delete local (`<ids>` or `--cleanup`)
**unarchive**: Restore archived queries (`<ids>`)

### Dashboards
**dashboards discover**: List your favorite dashboards (IDs + names + slugs)
**dashboards fetch**: Download dashboards (`<slugs>`)
**dashboards deploy**: Upload changes (`--all` for everything, or `<slugs>`)
**dashboards archive**: Archive dashboards + delete local (`<slugs>`)
**dashboards unarchive**: Restore archived dashboards (`<slugs>`)

**Note**: Only dashboards you've favorited in the Redash web UI will appear in `dashboards discover`.

Examples:
- `stmo-cli dashboards fetch firefox-desktop-on-steamos`
- `stmo-cli dashboards deploy --all`
- `stmo-cli dashboards archive bug-2006698---ccov-build-regression`

## File Format

**SQL**: `queries/{id}-{slug}.sql` - query text
**YAML**: `queries/{id}-{slug}.yaml` - metadata (name, data_source_id, parameters, visualizations)

## SQL Style

sqlfluff (BigQuery) enforced via pre-commit. Match existing `queries/*.sql` formatting.

## Query Creation

1. Create `0-{slug}.sql` + `0-{slug}.yaml` with `id: 0`
2. `stmo-cli deploy` → creates query in Redash
3. `stmo-cli fetch <new-id>` → sync with assigned ID

## Query/Dashboard Authoring

### Before Deploying SQL
Run `pre-commit run sqlfluff-lint --all-files` to catch formatting issues early (lowercase identifiers, proper indentation, etc).
