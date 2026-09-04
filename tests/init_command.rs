#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]

use std::env;
use std::sync::OnceLock;
use stmo_cli::commands::init::init;
use tempfile::TempDir;
use tokio::sync::Mutex;

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
async fn test_init_defaults_to_current_directory() {
    let _guard = get_test_lock().lock().await;
    let _temp_dir = TempWorkDir::new();

    // Make `pre-commit` unavailable for this call: `init()` otherwise ends by
    // running `pre-commit install`, which shells out to the system binary and
    // is unrelated to what this test checks (that a missing PATH argument
    // defaults to the current directory). This is the only test in this
    // binary, so overriding PATH process-wide is safe.
    let original_path = env::var("PATH").unwrap();
    unsafe {
        env::set_var("PATH", "/usr/bin:/bin");
    }

    init(None).unwrap();

    unsafe {
        env::set_var("PATH", original_path);
    }

    assert!(std::path::Path::new(".pre-commit-config.yaml").exists());
    assert!(std::path::Path::new("queries/.gitkeep").exists());
}
