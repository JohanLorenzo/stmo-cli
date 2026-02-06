# Schema Exploration Query Templates

These templates help you create queries to explore your database schema.

## Usage

1. Discover your data sources: `stmo-cli data-sources`
2. Create a new query in Redash (via web UI)
3. Set the appropriate data source
4. Copy the SQL template for your database type
5. Customize dataset/schema names for your environment
6. Save and run the query in Redash
7. Use `stmo-cli fetch <query_id>` to track the query locally
8. Use `stmo-cli execute <query_id>` to run it from the CLI

## Available Templates

- `bigquery_list_tables.sql` - Google BigQuery
- `postgres_list_tables.sql` - PostgreSQL/YugabyteDB
- `mysql_list_tables.sql` - MySQL/MariaDB

## Tips

- Start with approximate counts (faster) before using exact counts (slower)
- Filter by schema/dataset to reduce results
- Adjust LIMIT clauses based on your needs
- For large databases, consider parameterizing the queries
