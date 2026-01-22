#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

mod common;

use redash_tool::api::RedashClient;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, query_param};
use common::*;

#[tokio::test]
async fn test_refresh_query_success() {
    let mock_server = MockServer::start().await;

    mock_refresh_query(123, "test-job-id")
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let job = client.refresh_query(123, None).await.unwrap();

    assert_eq!(job.id, "test-job-id");
    assert_eq!(job.status, 1);
    assert!(job.query_result_id.is_none());
}

#[tokio::test]
async fn test_refresh_query_with_parameters() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/queries/123/results"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "job": {
                    "id": "test-job-id",
                    "status": 1,
                    "query_result_id": null,
                    "error": null
                }
            })
        ))
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let mut params = std::collections::HashMap::new();
    params.insert("start_date".to_string(), serde_json::json!("2025-01-01"));
    params.insert("channels".to_string(), serde_json::json!(["release", "beta"]));

    let job = client.refresh_query(123, Some(params)).await.unwrap();

    assert_eq!(job.id, "test-job-id");
    assert_eq!(job.status, 1);
}

#[tokio::test]
async fn test_poll_job_pending() {
    let mock_server = MockServer::start().await;

    mock_poll_job_pending("test-job-id")
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let job = client.poll_job("test-job-id").await.unwrap();

    assert_eq!(job.id, "test-job-id");
    assert_eq!(job.status, 1);
    assert!(job.query_result_id.is_none());
}

#[tokio::test]
async fn test_poll_job_success() {
    let mock_server = MockServer::start().await;

    mock_poll_job_success("test-job-id", 456)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let job = client.poll_job("test-job-id").await.unwrap();

    assert_eq!(job.id, "test-job-id");
    assert_eq!(job.status, 3);
    assert_eq!(job.query_result_id, Some(456));
}

#[tokio::test]
async fn test_poll_job_failure() {
    let mock_server = MockServer::start().await;

    mock_poll_job_failure("test-job-id", "Query execution failed")
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let job = client.poll_job("test-job-id").await.unwrap();

    assert_eq!(job.id, "test-job-id");
    assert_eq!(job.status, 4);
    assert_eq!(job.error, Some("Query execution failed".to_string()));
}

#[tokio::test]
async fn test_get_query_result_success() {
    let mock_server = MockServer::start().await;

    mock_get_query_result(123, 456)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let result = client.get_query_result(123, 456).await.unwrap();

    assert_eq!(result.id, 456);
    assert_eq!(result.data.columns.len(), 2);
    assert_eq!(result.data.columns[0].name, "col1");
    assert_eq!(result.data.rows.len(), 2);
    assert!((result.runtime - 1.5).abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_execute_query_with_polling_success() {
    let mock_server = MockServer::start().await;

    mock_refresh_query(123, "test-job-id")
        .expect(1)
        .mount(&mock_server)
        .await;

    mock_poll_job_success("test-job-id", 456)
        .expect(1)
        .mount(&mock_server)
        .await;

    mock_get_query_result(123, 456)
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let result = client.execute_query_with_polling(123, None, 10, 100).await.unwrap();

    assert_eq!(result.id, 456);
    assert_eq!(result.data.columns.len(), 2);
    assert_eq!(result.data.rows.len(), 2);
}

#[tokio::test]
async fn test_execute_query_with_polling_failure() {
    let mock_server = MockServer::start().await;

    mock_refresh_query(123, "test-job-id")
        .mount(&mock_server)
        .await;

    mock_poll_job_failure("test-job-id", "Syntax error in SQL")
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let result = client.execute_query_with_polling(123, None, 10, 100).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Query execution failed"));
    assert!(err.to_string().contains("Syntax error in SQL"));
}

#[tokio::test]
async fn test_execute_query_with_polling_timeout() {
    let mock_server = MockServer::start().await;

    mock_refresh_query(123, "test-job-id")
        .mount(&mock_server)
        .await;

    mock_poll_job_pending("test-job-id")
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let result = client.execute_query_with_polling(123, None, 1, 100).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("timed out"));
}

#[tokio::test]
async fn test_list_my_queries_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/queries/my"))
        .and(query_param("page", "1"))
        .and(query_param("page_size", "100"))
        .respond_with(ResponseTemplate::new(200).set_body_json(
            serde_json::json!({
                "results": [],
                "count": 0,
                "page": 1,
                "page_size": 100
            })
        ))
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let response = client.list_my_queries(1, 100).await.unwrap();

    assert_eq!(response.count, 0);
    assert_eq!(response.page, 1);
    assert_eq!(response.page_size, 100);
}

#[tokio::test]
async fn test_list_data_sources_success() {
    let mock_server = MockServer::start().await;

    mock_list_data_sources()
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let data_sources = client.list_data_sources().await.unwrap();

    assert_eq!(data_sources.len(), 2);
    assert_eq!(data_sources[0].id, 63);
    assert_eq!(data_sources[0].name, "Telemetry (BigQuery)");
    assert_eq!(data_sources[0].ds_type, "bigquery");
    assert_eq!(data_sources[1].id, 10);
    assert_eq!(data_sources[1].name, "Redash metadata");
    assert_eq!(data_sources[1].ds_type, "pg");
}

#[tokio::test]
async fn test_list_data_sources_empty() {
    let mock_server = MockServer::start().await;

    mock_list_data_sources_empty()
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let data_sources = client.list_data_sources().await.unwrap();

    assert_eq!(data_sources.len(), 0);
}

#[tokio::test]
async fn test_get_data_source_success() {
    let mock_server = MockServer::start().await;

    mock_get_data_source(63)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let data_source = client.get_data_source(63).await.unwrap();

    assert_eq!(data_source.id, 63);
    assert_eq!(data_source.name, "Test Data Source");
    assert_eq!(data_source.ds_type, "bigquery");
    assert_eq!(data_source.description, Some("Test description".to_string()));
}

#[tokio::test]
async fn test_get_data_source_not_found() {
    let mock_server = MockServer::start().await;

    mock_get_data_source_not_found(999)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let result = client.get_data_source(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_data_source_schema_success() {
    let mock_server = MockServer::start().await;

    mock_get_data_source_schema(63)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let schema = client.get_data_source_schema(63, false).await.unwrap();

    assert_eq!(schema.schema.len(), 2);
    assert_eq!(schema.schema[0].name, "table1");
    assert_eq!(schema.schema[0].columns.len(), 3);
    assert_eq!(schema.schema[0].columns[0].name, "col1");
    assert_eq!(schema.schema[0].columns[0].column_type, "STRING");
    assert_eq!(schema.schema[1].name, "table2");
    assert_eq!(schema.schema[1].columns.len(), 2);
    assert_eq!(schema.schema[1].columns[0].name, "id");
    assert_eq!(schema.schema[1].columns[0].column_type, "INTEGER");
}

#[tokio::test]
async fn test_get_data_source_schema_unauthorized() {
    let mock_server = MockServer::start().await;

    mock_get_data_source_schema_unauthorized(63)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();
    let result = client.get_data_source_schema(63, false).await;

    assert!(result.is_err());
}
