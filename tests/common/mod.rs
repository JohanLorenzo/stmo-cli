#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

pub struct TestContext {
    pub mock_server: MockServer,
    pub temp_dir: TempDir,
    pub queries_dir: PathBuf,
}

impl TestContext {
    pub async fn new() -> Self {
        let mock_server = MockServer::start().await;
        let temp_dir = TempDir::new().unwrap();
        let queries_dir = temp_dir.path().join("queries");
        fs::create_dir(&queries_dir).unwrap();

        Self {
            mock_server,
            temp_dir,
            queries_dir,
        }
    }

    pub fn base_url(&self) -> String {
        self.mock_server.uri()
    }

    pub fn create_query_files(
        &self,
        id: u64,
        slug: &str,
        sql: &str,
        yaml_content: &str,
    ) {
        let sql_path = self.queries_dir.join(format!("{id}-{slug}.sql"));
        let yaml_path = self.queries_dir.join(format!("{id}-{slug}.yaml"));

        fs::write(sql_path, sql).unwrap();
        fs::write(yaml_path, yaml_content).unwrap();
    }
}

pub fn mock_refresh_query(query_id: u64, job_id: &str) -> Mock {
    Mock::given(method("POST"))
        .and(path(format!("/api/queries/{query_id}/results")))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "job": {
                    "id": job_id,
                    "status": 1,
                    "query_result_id": null,
                    "error": null
                }
            })
        ))
}

pub fn mock_poll_job_pending(job_id: &str) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/jobs/{job_id}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "job": {
                    "id": job_id,
                    "status": 1,
                    "query_result_id": null,
                    "error": null
                }
            })
        ))
}

pub fn mock_poll_job_success(job_id: &str, result_id: u64) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/jobs/{job_id}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "job": {
                    "id": job_id,
                    "status": 3,
                    "query_result_id": result_id,
                    "error": null
                }
            })
        ))
}

pub fn mock_poll_job_failure(job_id: &str, error_msg: &str) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/jobs/{job_id}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "job": {
                    "id": job_id,
                    "status": 4,
                    "query_result_id": null,
                    "error": error_msg
                }
            })
        ))
}

pub fn mock_get_query_result(query_id: u64, result_id: u64) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/queries/{query_id}/results/{result_id}.json")))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "query_result": {
                    "id": result_id,
                    "data": {
                        "columns": [
                            {"name": "col1", "type": "string"},
                            {"name": "col2", "type": "integer"}
                        ],
                        "rows": [
                            {"col1": "value1", "col2": 123},
                            {"col1": "value2", "col2": 456}
                        ]
                    },
                    "runtime": 1.5,
                    "retrieved_at": "2026-01-21T10:00:00"
                }
            })
        ))
}

pub fn mock_list_my_queries(page: u32, page_size: u32, total_count: u64) -> Mock {
    Mock::given(method("GET"))
        .and(path("/api/queries/my"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "results": [],
                "count": total_count,
                "page": page,
                "page_size": page_size
            })
        ))
}

pub fn mock_list_data_sources() -> Mock {
    Mock::given(method("GET"))
        .and(path("/api/data_sources"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!([
                {
                    "id": 63,
                    "name": "Telemetry (BigQuery)",
                    "type": "bigquery",
                    "description": null,
                    "syntax": "sql",
                    "paused": 0,
                    "pause_reason": null,
                    "view_only": false,
                    "queue_name": "bq_queries",
                    "scheduled_queue_name": "bq_scheduled_queries",
                    "groups": {"2": false},
                    "options": {}
                },
                {
                    "id": 10,
                    "name": "Redash metadata",
                    "type": "pg",
                    "description": null,
                    "syntax": "sql",
                    "paused": 0,
                    "pause_reason": null,
                    "view_only": false,
                    "queue_name": "queries",
                    "scheduled_queue_name": "scheduled_queries",
                    "groups": {"2": false},
                    "options": {}
                }
            ])
        ))
}

pub fn mock_list_data_sources_empty() -> Mock {
    Mock::given(method("GET"))
        .and(path("/api/data_sources"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!([])
        ))
}

pub fn mock_get_data_source(data_source_id: u64) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/data_sources/{data_source_id}")))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "id": data_source_id,
                "name": "Test Data Source",
                "type": "bigquery",
                "description": "Test description",
                "syntax": "sql",
                "paused": 0,
                "pause_reason": null,
                "view_only": false,
                "queue_name": "queries",
                "scheduled_queue_name": "scheduled_queries",
                "groups": {},
                "options": {}
            })
        ))
}

pub fn mock_get_data_source_not_found(data_source_id: u64) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/data_sources/{data_source_id}")))
        .respond_with(ResponseTemplate::new(404))
}

pub fn mock_get_data_source_schema(data_source_id: u64) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/data_sources/{data_source_id}/schema")))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "schema": [
                    {
                        "name": "table1",
                        "columns": ["col1", "col2", "col3"]
                    },
                    {
                        "name": "table2",
                        "columns": ["id", "name"]
                    }
                ]
            })
        ))
}

pub fn mock_get_data_source_schema_unauthorized(data_source_id: u64) -> Mock {
    Mock::given(method("GET"))
        .and(path(format!("/api/data_sources/{data_source_id}/schema")))
        .respond_with(ResponseTemplate::new(403))
}
