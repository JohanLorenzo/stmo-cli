-- List all tables with row counts in BigQuery
-- Uses INFORMATION_SCHEMA for approximate counts
-- Note: row_count is approximate and may not be current

SELECT
    table_schema,
    table_name,
    creation_time,
    row_count,
    ROUND(size_bytes / POW(10, 9), 2) AS size_gb
FROM `mozdata.INFORMATION_SCHEMA.TABLE_STORAGE`
WHERE table_type = 'BASE TABLE'
ORDER BY row_count DESC
LIMIT 100
