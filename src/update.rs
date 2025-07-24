use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use log::{info, warn};
use reqwest::Client;
use semver::Version;
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

const GITHUB_API_URL: &str = "https://api.github.com/repos/Mahmoud-Emad/commi/releases/latest";
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn check_for_updates() -> Result<bool> {
    let client = Client::new();

    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "commi-updater")
        .send()
        .await
        .context("Failed to check for updates")?;

    if !response.status().is_success() {
        return Ok(false); // Silently fail update checks
    }

    let release: GitHubRelease = response
        .json()
        .await
        .context("Failed to parse release information")?;

    let latest_version = release.tag_name.trim_start_matches('v');
    let current_version = CURRENT_VERSION;

    // Use semantic version comparison
    match (
        Version::parse(current_version),
        Version::parse(latest_version),
    ) {
        (Ok(current), Ok(latest)) => Ok(latest > current),
        _ => {
            // Fallback to string comparison if parsing fails
            Ok(latest_version != current_version)
        }
    }
}

/// Get the latest version from GitHub releases with caching
pub async fn get_latest_version() -> Result<String> {
    // Try to get cached version first
    if let Ok(cached_version) = get_cached_version().await {
        return Ok(cached_version);
    }

    // Fetch from GitHub if cache is stale or missing
    let version = fetch_latest_version_from_github().await?;

    // Update cache
    if let Err(e) = update_version_cache(&version).await {
        log::warn!("Failed to update version cache: {}", e);
    }

    Ok(version)
}

/// Fetch the latest version directly from GitHub
async fn fetch_latest_version_from_github() -> Result<String> {
    let client = Client::new();

    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "commi-updater")
        .send()
        .await
        .context("Failed to check for updates")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch latest version from GitHub");
    }

    let release: GitHubRelease = response
        .json()
        .await
        .context("Failed to parse release information")?;

    Ok(release.tag_name.trim_start_matches('v').to_string())
}

/// Get cached version if it's less than 1 hour old
async fn get_cached_version() -> Result<String> {
    use crate::config::Config;

    let config = Config::load_toml_config()?;

    if let (Some(cached_version), Some(last_check)) =
        (config.cache.latest_version, config.cache.last_version_check)
    {
        let now = Utc::now();
        let age = now.signed_duration_since(last_check);

        // Cache is valid for 1 hour
        if age < Duration::hours(1) {
            return Ok(cached_version);
        }
    }

    anyhow::bail!("Cache is stale or missing")
}

/// Update the version cache
async fn update_version_cache(version: &str) -> Result<()> {
    use crate::config::Config;

    let mut config = Config::load_toml_config()?;
    config.cache.latest_version = Some(version.to_string());
    config.cache.last_version_check = Some(Utc::now());

    Config::save_toml_config(&config)?;
    Ok(())
}

/// Get the current version
pub fn get_current_version() -> String {
    CURRENT_VERSION.to_string()
}

/// Compare two version strings using semantic versioning
pub fn compare_versions(current: &str, latest: &str) -> Result<std::cmp::Ordering> {
    match (Version::parse(current), Version::parse(latest)) {
        (Ok(current_ver), Ok(latest_ver)) => Ok(current_ver.cmp(&latest_ver)),
        _ => {
            // Fallback to string comparison if parsing fails
            Ok(current.cmp(latest))
        }
    }
}

pub async fn update_binary() -> Result<()> {
    info!("Checking for updates...");

    let client = Client::new();

    let response = client
        .get(GITHUB_API_URL)
        .header("User-Agent", "commi-updater")
        .send()
        .await
        .context("Failed to fetch release information")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to fetch release information from GitHub");
    }

    let release: GitHubRelease = response
        .json()
        .await
        .context("Failed to parse release information")?;

    let latest_version = release.tag_name.trim_start_matches('v');
    let current_version = CURRENT_VERSION;

    if latest_version == current_version {
        info!("You're already running the latest version (v{current_version})");
        return Ok(());
    }

    info!("Found new version: v{latest_version} (current: v{current_version})");

    // Determine the correct binary name for the current platform
    let binary_name = get_platform_binary_name()?;

    // Find the matching asset
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == binary_name)
        .context(format!("No binary found for platform: {binary_name}"))?;

    info!("Downloading {binary_name}...");

    // Download the binary
    let binary_response = client
        .get(&asset.browser_download_url)
        .send()
        .await
        .context("Failed to download binary")?;

    if !binary_response.status().is_success() {
        anyhow::bail!("Failed to download binary");
    }

    let binary_data = binary_response
        .bytes()
        .await
        .context("Failed to read binary data")?;

    // Get the current executable path
    let current_exe = env::current_exe().context("Failed to get current executable path")?;

    // Create a backup of the current binary
    let backup_path = format!("{}.backup", current_exe.display());
    fs::copy(&current_exe, &backup_path).context("Failed to create backup of current binary")?;

    info!("💾 Created backup at: {backup_path}");

    // Write the new binary
    fs::write(&current_exe, binary_data).context("Failed to write new binary")?;

    // Make it executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&current_exe)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&current_exe, perms)?;
    }

    info!("Successfully updated to v{latest_version}!");
    info!("Please restart commi to use the new version");

    // Clean up backup
    if let Err(e) = fs::remove_file(&backup_path) {
        warn!("Failed to remove backup file: {e}");
    }

    Ok(())
}

fn get_platform_binary_name() -> Result<String> {
    let os = env::consts::OS;
    let arch = env::consts::ARCH;

    let binary_name = match (os, arch) {
        ("linux", "x86_64") => "commi-x86_64-unknown-linux-gnu",
        ("macos", "x86_64") => "commi-x86_64-apple-darwin",
        ("macos", "aarch64") => "commi-aarch64-apple-darwin",
        ("windows", "x86_64") => "commi-x86_64-pc-windows-msvc.exe",
        _ => anyhow::bail!("Unsupported platform: {os}-{arch}"),
    };

    Ok(binary_name.to_string())
}
