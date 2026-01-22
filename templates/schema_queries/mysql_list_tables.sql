-- List all tables with row counts in MySQL/MariaDB
-- Uses INFORMATION_SCHEMA.TABLES

SELECT
    table_schema,
    table_name,
    create_time,
    table_rows AS row_count,
    ROUND(data_length / 1024 / 1024 / 1024, 2) AS size_gb
FROM
    information_schema.tables
WHERE
    table_schema NOT IN (
        'information_schema', 'mysql', 'performance_schema', 'sys'
    )
    AND table_type = 'BASE TABLE'
ORDER BY table_rows DESC
LIMIT 100
