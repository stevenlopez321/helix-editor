//! Static build info for Helix Editor.

pub const BUILD_VERSION: &str  = "v4.5.19";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "13eba920e30e";
pub const BUILD_TIMESTAMP: u64 = 1754836616;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Helix Editor", BUILD_VERSION, BUILD_CHANNEL)
}
