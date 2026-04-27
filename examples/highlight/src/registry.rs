//! Remote theme registry — fetches the shiki `tm-themes` catalog from
//! jsdelivr and caches both the catalog (TTL 24h) and individual theme
//! JSONs (forever — themes are immutable per slug+version) on disk under
//! the platform cache dir.
//!
//! Endpoints:
//!
//! - `https://data.jsdelivr.com/v1/package/npm/tm-themes` — version metadata
//! - `https://data.jsdelivr.com/v1/package/npm/tm-themes@{version}` — file tree
//! - `https://cdn.jsdelivr.net/npm/tm-themes/themes/{slug}.json` — theme JSON
//!
//! All errors surface as `String` so they can flow through iced messages
//! (which require `Clone`).

use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};

const CATALOG_TTL: Duration = Duration::from_secs(60 * 60 * 24);
const META_BASE: &str = "https://data.jsdelivr.com/v1/package/npm/tm-themes";
const CDN_BASE: &str = "https://cdn.jsdelivr.net/npm/tm-themes";

/// One theme available in the registry. `slug` is the filename stem used
/// for fetching; `display_name` is the human-friendly form derived from
/// the slug.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub slug: String,
    pub display_name: String,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display_name)
    }
}

impl Entry {
    fn from_filename(filename: &str) -> Option<Self> {
        let slug = filename.strip_suffix(".json")?.to_owned();
        let display_name = slug
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    Some(c) => c.to_uppercase().chain(chars).collect::<String>(),
                    None => String::new(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        Some(Self { slug, display_name })
    }
}

#[derive(Deserialize)]
struct Versions {
    tags: Tags,
}

#[derive(Deserialize)]
struct Tags {
    latest: String,
}

#[derive(Deserialize)]
struct FileTree {
    files: Vec<FileNode>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum FileNode {
    Directory { name: String, files: Vec<FileNode> },
    File { name: String },
}

fn cache_root() -> Result<PathBuf, String> {
    let dir = dirs::cache_dir()
        .ok_or_else(|| "no platform cache directory available".to_owned())?
        .join("icedxdy")
        .join("highlight");
    std::fs::create_dir_all(dir.join("themes")).map_err(|e| e.to_string())?;
    Ok(dir)
}

/// Load the theme catalog. Returns the disk-cached copy if it exists and
/// is fresher than the TTL; otherwise fetches the remote tree and writes
/// the result to disk before returning.
pub async fn fetch_catalog() -> Result<Vec<Entry>, String> {
    let root = cache_root()?;
    let path = root.join("catalog.json");

    if let Ok(meta) = std::fs::metadata(&path)
        && let Ok(modified) = meta.modified()
        && let Ok(elapsed) = modified.elapsed()
        && elapsed < CATALOG_TTL
        && let Ok(bytes) = std::fs::read(&path)
        && let Ok(entries) = serde_json::from_slice::<Vec<Entry>>(&bytes)
    {
        return Ok(entries);
    }

    let entries = fetch_catalog_remote().await?;

    if let Ok(bytes) = serde_json::to_vec(&entries) {
        let _ = std::fs::write(&path, bytes);
    }
    Ok(entries)
}

async fn fetch_catalog_remote() -> Result<Vec<Entry>, String> {
    let client = reqwest::Client::new();

    let versions: Versions = client
        .get(META_BASE)
        .send()
        .await
        .map_err(|e| format!("catalog version: {e}"))?
        .json()
        .await
        .map_err(|e| format!("catalog version parse: {e}"))?;

    let tree: FileTree = client
        .get(format!("{META_BASE}@{}", versions.tags.latest))
        .send()
        .await
        .map_err(|e| format!("catalog tree: {e}"))?
        .json()
        .await
        .map_err(|e| format!("catalog tree parse: {e}"))?;

    let mut entries: Vec<Entry> = tree
        .files
        .into_iter()
        .filter_map(|n| match n {
            FileNode::Directory { name, files } if name == "themes" => Some(files),
            _ => None,
        })
        .flatten()
        .filter_map(|n| match n {
            FileNode::File { name } => Entry::from_filename(&name),
            _ => None,
        })
        .collect();

    entries.sort_by(|a, b| a.display_name.cmp(&b.display_name));
    Ok(entries)
}

/// Fetch a single theme's JSON. Cached forever once written — themes are
/// immutable per slug at any point in time, so a cached copy is always
/// good enough.
pub async fn fetch_theme(slug: String) -> Result<String, String> {
    let root = cache_root()?;
    let path = root.join("themes").join(format!("{slug}.json"));

    if let Ok(content) = std::fs::read_to_string(&path) {
        return Ok(content);
    }

    let body = reqwest::get(format!("{CDN_BASE}/themes/{slug}.json"))
        .await
        .map_err(|e| format!("fetch {slug}: {e}"))?
        .error_for_status()
        .map_err(|e| format!("fetch {slug}: {e}"))?
        .text()
        .await
        .map_err(|e| format!("fetch {slug}: {e}"))?;

    let _ = std::fs::write(&path, &body);
    Ok(body)
}
