-- List all tables with row counts in PostgreSQL
-- Uses pg_stat_user_tables for approximate counts

SELECT
    schemaname AS table_schema,
    tablename AS table_name,
    n_live_tup AS row_count,
    pg_size_pretty(
        pg_total_relation_size(schemaname || '.' || tablename)
    ) AS size
FROM pg_stat_user_tables
WHERE schemaname NOT IN ('pg_catalog', 'information_schema')
ORDER BY n_live_tup DESC
LIMIT 100
