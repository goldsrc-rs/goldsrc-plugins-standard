//! Sandboxed path resolution and file inspection for coreutils.
//!
//! Exposes virtual filesystem primitives delegating directly to the Host VFS WIT interface.
//! Plugins have 0 hardcoded paths, 0 OS assumptions, and zero knowledge of mod directory layout.

#[cfg(not(test))]
use goldsrc_api::bindings::goldsrc::engine::api;
use goldsrc_api::bindings::goldsrc::engine::api::DirEntry;

/// Safe VFS helper delegating to Host VFS WIT interface.
pub struct SafeVfs;

impl SafeVfs {
    /// Read text file via Host VFS.
    pub fn read_to_string(rel_path: &str) -> Result<String, String> {
        if rel_path.contains("..") {
            return Err("Access denied: path traversal not permitted".to_string());
        }
        #[cfg(test)]
        {
            std::fs::read_to_string(rel_path).map_err(|e| e.to_string())
        }
        #[cfg(not(test))]
        {
            api::host_fs_read_text(rel_path)
        }
    }

    /// Read binary file bytes via Host VFS.
    pub fn read_bytes(rel_path: &str) -> Result<Vec<u8>, String> {
        if rel_path.contains("..") {
            return Err("Access denied: path traversal not permitted".to_string());
        }
        #[cfg(test)]
        {
            std::fs::read(rel_path).map_err(|e| e.to_string())
        }
        #[cfg(not(test))]
        {
            api::host_fs_read_bytes(rel_path)
        }
    }

    /// List directory entries via Host VFS.
    pub fn list_dir(rel_path: &str) -> Result<Vec<DirEntry>, String> {
        if rel_path.contains("..") {
            return Err("Access denied: path traversal not permitted".to_string());
        }
        #[cfg(test)]
        {
            let rd = std::fs::read_dir(rel_path).map_err(|e| e.to_string())?;
            let mut entries = Vec::new();
            for item in rd.flatten() {
                let name = item.file_name().to_string_lossy().to_string();
                let is_dir = item.file_type().map(|t| t.is_dir()).unwrap_or(false);
                let size = item.metadata().map(|m| m.len()).unwrap_or(0);
                entries.push(DirEntry { name, is_dir, size });
            }
            Ok(entries)
        }
        #[cfg(not(test))]
        {
            api::host_fs_list_dir(rel_path)
        }
    }

    /// Recursively collect matching file paths under directory.
    pub fn find_files(dir: &str, ext_pattern: Option<&str>) -> Vec<String> {
        let mut results = Vec::new();
        if let Ok(entries) = Self::list_dir(dir) {
            for entry in entries {
                let child_path = if dir == "." || dir.is_empty() {
                    entry.name.clone()
                } else {
                    format!("{dir}/{}", entry.name)
                };

                if entry.is_dir {
                    results.extend(Self::find_files(&child_path, ext_pattern));
                } else if let Some(pattern) = ext_pattern {
                    let pattern_ext = pattern.trim_start_matches("*.");
                    if entry.name.ends_with(&format!(".{pattern_ext}")) || entry.name == pattern {
                        results.push(child_path);
                    }
                } else {
                    results.push(child_path);
                }
            }
        }
        results
    }

    /// Recursively calculate directory size in bytes.
    pub fn du_calc(dir: &str) -> (u64, Vec<(String, u64)>) {
        let mut total = 0u64;
        let mut items = Vec::new();
        if let Ok(entries) = Self::list_dir(dir) {
            for entry in entries {
                let child_path = if dir == "." || dir.is_empty() {
                    entry.name.clone()
                } else {
                    format!("{dir}/{}", entry.name)
                };
                if entry.is_dir {
                    let (sub_total, sub_items) = Self::du_calc(&child_path);
                    total += sub_total;
                    items.extend(sub_items);
                    items.push((child_path, sub_total));
                } else {
                    total += entry.size;
                    items.push((child_path, entry.size));
                }
            }
        }
        (total, items)
    }
}
