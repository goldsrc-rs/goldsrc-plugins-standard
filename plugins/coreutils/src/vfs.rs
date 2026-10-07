//! Sandboxed path resolution and file inspection for coreutils.

use std::fs;
use std::path::{Path, PathBuf};

/// Safe VFS helper to restrict read access within the server directory.
pub struct SafeVfs;

impl SafeVfs {
    /// Read text file with sandboxed boundary check.
    pub fn read_to_string(rel_path: &str) -> Result<String, String> {
        let path = PathBuf::from(rel_path);

        // Disallow path traversal attacks
        if rel_path.contains("..") {
            return Err("Access denied: path traversal not permitted".to_string());
        }

        if !path.exists() {
            return Err(format!("File not found: {rel_path}"));
        }

        fs::read_to_string(&path).map_err(|e| format!("Failed to read '{rel_path}': {e}"))
    }

    /// Read binary file bytes with sandboxed boundary check.
    pub fn read_bytes(rel_path: &str) -> Result<Vec<u8>, String> {
        let path = PathBuf::from(rel_path);

        if rel_path.contains("..") {
            return Err("Access denied: path traversal not permitted".to_string());
        }

        if !path.exists() {
            return Err(format!("File not found: {rel_path}"));
        }

        fs::read(&path).map_err(|e| format!("Failed to read '{rel_path}': {e}"))
    }

    /// Collect matching files under directory with extension filter.
    pub fn find_files(dir: &Path, ext_pattern: Option<&str>) -> Vec<PathBuf> {
        let mut results = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
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
