//! Sandboxed path resolution and file inspection for coreutils.

use std::fs;
use std::path::{Path, PathBuf};

/// Safe VFS helper to restrict read access within the server directory.
pub struct SafeVfs;

impl SafeVfs {
    /// Resolve candidate relative path checking addon dir, mod dir, and server root.
    pub fn resolve_path(rel_path: &str) -> Option<PathBuf> {
        let p = Path::new(rel_path);
        // Candidate 1: directly as given
        if p.exists() {
            return Some(p.to_path_buf());
        }

        // Candidate 2: inside cstrike/addons/goldsrc/
        let in_addon = Path::new("cstrike/addons/goldsrc").join(p);
        if in_addon.exists() {
            return Some(in_addon);
        }

        // Candidate 3: inside addons/goldsrc/
        let in_mod_addon = Path::new("addons/goldsrc").join(p);
        if in_mod_addon.exists() {
            return Some(in_mod_addon);
        }

        // Candidate 4: inside cstrike/
        let in_cstrike = Path::new("cstrike").join(p);
        if in_cstrike.exists() {
            return Some(in_cstrike);
        }

        None
    }

    /// Read text file with sandboxed boundary check.
    pub fn read_to_string(rel_path: &str) -> Result<String, String> {
        // Disallow path traversal attacks
        if rel_path.contains("..") {
            return Err("Access denied: path traversal not permitted".to_string());
        }

        let path = Self::resolve_path(rel_path)
            .ok_or_else(|| format!("File not found: {rel_path}"))?;

        fs::read_to_string(&path).map_err(|e| format!("Failed to read '{rel_path}': {e}"))
    }

    /// Read binary file bytes with sandboxed boundary check.
    pub fn read_bytes(rel_path: &str) -> Result<Vec<u8>, String> {
        if rel_path.contains("..") {
            return Err("Access denied: path traversal not permitted".to_string());
        }

        let path = Self::resolve_path(rel_path)
            .ok_or_else(|| format!("File not found: {rel_path}"))?;

        fs::read(&path).map_err(|e| format!("Failed to read '{rel_path}': {e}"))
    }

    /// Collect matching files under directory with extension filter.
    pub fn find_files(dir: &Path, ext_pattern: Option<&str>) -> Vec<PathBuf> {
        let mut results = Vec::new();
        let target_dir = if dir.exists() {
            dir.to_path_buf()
        } else if let Some(resolved) = Self::resolve_path(&dir.to_string_lossy()) {
            resolved
        } else {
            dir.to_path_buf()
        };

        if let Ok(entries) = fs::read_dir(&target_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    results.extend(Self::find_files(&path, ext_pattern));
                } else if let Some(pattern) = ext_pattern {
                    let pattern_ext = pattern.trim_start_matches("*.");
                    if path.extension().and_then(|e| e.to_str()) == Some(pattern_ext) {
                        results.push(path);
                    }
                } else {
                    results.push(path);
                }
            }
        }
        results
    }
}
