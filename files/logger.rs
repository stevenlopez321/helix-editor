//! Thin wrapper around `log` that writes to %LOCALAPPDATA%\Helix Editor\logs\.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const LOG_FILENAME: &str = "helix_editor.log";
const MAX_LOG_BYTES: u64 = 2097152;

pub fn log_path() -> PathBuf {
    let base = std::env::var("LOCALAPPDATA").unwrap_or_else(|_| String::from("."));
    PathBuf::from(base).join("Helix Editor").join("logs").join(LOG_FILENAME)
}

pub fn append(level: &str, message: &str) -> std::io::Result<()> {
    let path = log_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    rotate_if_needed(&path)?;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    let ts = chrono_like_ts();
    writeln!(file, "{} [{}] {}", ts, level, message)?;
    Ok(())
}

fn rotate_if_needed(path: &std::path::Path) -> std::io::Result<()> {
    match std::fs::metadata(path) {
        Ok(m) if m.len() > MAX_LOG_BYTES => {
            let backup = path.with_extension("log.old");
            let _ = std::fs::remove_file(&backup);
            std::fs::rename(path, backup)?;
        }
        _ => {}
    }
    Ok(())
}

fn chrono_like_ts() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("ts={}", now)
}
