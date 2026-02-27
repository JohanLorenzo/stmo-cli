#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

mod common;

use stmo_cli::api::RedashClient;
use common::*;
use tempfile::TempDir;
use std::env;
use tokio::sync::Mutex;
use std::sync::OnceLock;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn get_test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

struct TempWorkDir {
    _temp_dir: TempDir,
    original_dir: std::path::PathBuf,
}

impl TempWorkDir {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();
        Self {
            _temp_dir: temp_dir,
            original_dir,
        }
    }
}

impl Drop for TempWorkDir {
    fn drop(&mut self) {
        env::set_current_dir(&self.original_dir).ok();
    }
}

#[tokio::test]
async fn test_deploy_new_query_with_id_zero() {
    let _guard = get_test_lock().lock().await;
    let _temp_dir = TempWorkDir::new();
    let mock_server = wiremock::MockServer::start().await;

    mock_create_query(42, "Test Query")
        .mount(&mock_server)
        .await;

    mock_get_query(42, "Test Query", false)
        .mount(&mock_server)
        .await;

    let client = RedashClient::new(mock_server.uri(), "test-key").unwrap();

    std::fs::create_dir_all("queries").unwrap();
    std::fs::write("queries/0-test-query.sql", "SELECT 1").unwrap();
    std::fs::write(
        "queries/0-test-query.yaml",
        "id: 0\nname: Test Query\ndescription: null\ndata_source_id: 63\nschedule: null\noptions:\n  parameters: []\nvisualizations: []\ntags: null\n",
    )
    .unwrap();

    let result = stmo_cli::commands::deploy::deploy(&client, vec![0], false).await;

    assert!(result.is_ok(), "Deploy failed: {:?}", result.err());

    assert!(
        !std::path::Path::new("queries/0-test-query.sql").exists(),
        "Old 0-*.sql file should be removed after creation"
    );
    assert!(
        !std::path::Path::new("queries/0-test-query.yaml").exists(),
        "Old 0-*.yaml file should be removed after creation"
    );

    assert!(
        std::path::Path::new("queries/42-test-query.sql").exists(),
        "New SQL file with server ID should be created"
    );
    assert!(
        std::path::Path::new("queries/42-test-query.yaml").exists(),
        "New YAML file with server ID should be created"
    );

    let yaml_content = std::fs::read_to_string("queries/42-test-query.yaml").unwrap();
    assert!(yaml_content.contains("id: 42"), "YAML should contain the new ID");
}
