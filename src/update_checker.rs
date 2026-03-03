use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CACHE_MAX_AGE_SECS: u64 = 86400;
const FETCH_TIMEOUT_SECS: u64 = 2;

#[derive(Debug, Serialize, Deserialize)]
struct VersionCache {
    latest_version: String,
    last_checked: u64,
}

#[must_use]
pub fn installed_via_cargo() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let Ok(exe) = exe.canonicalize() else {
        return false;
    };
    let Some(home) = dirs::home_dir() else {
        return false;
    };
    let cargo_bin = home.join(".cargo").join("bin");
    exe.starts_with(cargo_bin)
}

fn cache_path() -> Option<PathBuf> {
    Some(dirs::cache_dir()?.join("stmo-cli").join("version-check.json"))
}

fn read_cache() -> Option<VersionCache> {
    let path = cache_path()?;
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}

fn write_cache(version: &str, timestamp: u64) {
    let Some(path) = cache_path() else { return };
    let Some(parent) = path.parent() else { return };
    let _ = std::fs::create_dir_all(parent);
    let cache = VersionCache {
        latest_version: version.to_string(),
        last_checked: timestamp,
    };
    if let Ok(data) = serde_json::to_string(&cache) {
        let _ = std::fs::write(path, data);
    }
}

fn should_check(cache: Option<&VersionCache>) -> bool {
    let Some(cache) = cache else { return true };
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    now.saturating_sub(cache.last_checked) > CACHE_MAX_AGE_SECS
}

async fn fetch_latest_version(base_url: &str) -> Option<String> {
    let url = format!("{base_url}/api/v1/crates/stmo-cli");
    let user_agent = format!("stmo-cli/{CURRENT_VERSION}");
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(FETCH_TIMEOUT_SECS))
        .build()
        .ok()?;
    let resp = client
        .get(&url)
        .header("User-Agent", user_agent)
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        return None;
    }
    let body: serde_json::Value = resp.json().await.ok()?;
    body["crate"]["max_version"]
        .as_str()
        .map(str::to_string)
}

#[must_use]
pub fn is_newer(latest: &str, current: &str) -> bool {
    let Ok(latest) = latest.parse::<semver::Version>() else {
        return false;
    };
    let Ok(current) = current.parse::<semver::Version>() else {
        return false;
    };
    latest > current
}

pub async fn check_for_update_from(base_url: &str) -> Option<String> {
    let cache = read_cache();
    let latest = if should_check(cache.as_ref()) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let version = fetch_latest_version(base_url).await?;
        write_cache(&version, now);
        version
    } else {
        cache?.latest_version
    };

    if is_newer(&latest, CURRENT_VERSION) {
        Some(latest)
    } else {
        None
    }
}

pub async fn check_and_auto_update() {
    if !installed_via_cargo() {
        return;
    }

    let Some(latest) = check_for_update_from("https://crates.io").await else {
        return;
    };

    eprintln!("Updating stmo-cli {CURRENT_VERSION} → {latest}...");

    let status = std::process::Command::new("cargo")
        .args(["install", "stmo-cli"])
        .status();

    match status {
        Ok(s) if s.success() => {
            let args: Vec<String> = std::env::args().collect();
            #[cfg(unix)]
            {
                use std::os::unix::process::CommandExt as _;
                let mut cmd = std::process::Command::new(&args[0]);
                cmd.args(&args[1..]);
                let err = cmd.exec();
                eprintln!("Failed to re-exec: {err}");
            }
            #[cfg(not(unix))]
            {
                let _ = std::process::Command::new(&args[0])
                    .args(&args[1..])
                    .status();
                std::process::exit(0);
            }
        }
        _ => {
            eprintln!("Update failed, continuing with current version.");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn is_newer_detects_patch_bump() {
        assert!(is_newer("0.3.1", "0.3.0"));
    }

    #[test]
    fn is_newer_detects_minor_bump() {
        assert!(is_newer("0.4.0", "0.3.0"));
    }

    #[test]
    fn is_newer_detects_major_bump() {
        assert!(is_newer("1.0.0", "0.3.0"));
    }

    #[test]
    fn is_newer_same_version_false() {
        assert!(!is_newer("0.3.0", "0.3.0"));
    }

    #[test]
    fn is_newer_older_version_false() {
        assert!(!is_newer("0.2.0", "0.3.0"));
    }

    #[test]
    fn is_newer_invalid_latest_false() {
        assert!(!is_newer("not-a-version", "0.3.0"));
    }

    #[test]
    fn is_newer_invalid_current_false() {
        assert!(!is_newer("0.4.0", "not-a-version"));
    }

    #[test]
    fn should_check_returns_true_when_no_cache() {
        assert!(should_check(None));
    }

    #[test]
    fn should_check_returns_true_for_stale_cache() {
        let stale = VersionCache {
            latest_version: "0.3.0".to_string(),
            last_checked: 0,
        };
        assert!(should_check(Some(&stale)));
    }

    #[test]
    fn should_check_returns_false_for_fresh_cache() {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let fresh = VersionCache {
            latest_version: "0.3.0".to_string(),
            last_checked: now,
        };
        assert!(!should_check(Some(&fresh)));
    }

    #[test]
    fn cache_roundtrip() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("version-check.json");

        let cache = VersionCache {
            latest_version: "1.2.3".to_string(),
            last_checked: 9999,
        };
        std::fs::write(&path, serde_json::to_string(&cache).unwrap()).unwrap();

        let loaded: VersionCache =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(loaded.latest_version, "1.2.3");
        assert_eq!(loaded.last_checked, 9999);
    }
}
