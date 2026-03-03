#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use stmo_cli::update_checker::check_for_update_from;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn crates_response(version: &str) -> serde_json::Value {
    serde_json::json!({
        "crate": {
            "max_version": version
        }
    })
}

#[tokio::test]
async fn newer_version_returns_some() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/crates/stmo-cli"))
        .respond_with(ResponseTemplate::new(200).set_body_json(crates_response("99.0.0")))
        .mount(&mock_server)
        .await;

    let result = check_for_update_from(&mock_server.uri()).await;
    assert_eq!(result, Some("99.0.0".to_string()));
}

#[tokio::test]
async fn same_version_returns_none() {
    let mock_server = MockServer::start().await;
    let current = env!("CARGO_PKG_VERSION");

    Mock::given(method("GET"))
        .and(path("/api/v1/crates/stmo-cli"))
        .respond_with(ResponseTemplate::new(200).set_body_json(crates_response(current)))
        .mount(&mock_server)
        .await;

    let result = check_for_update_from(&mock_server.uri()).await;
    assert_eq!(result, None);
}

#[tokio::test]
async fn server_error_returns_none() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/crates/stmo-cli"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;

    let result = check_for_update_from(&mock_server.uri()).await;
    assert_eq!(result, None);
}
