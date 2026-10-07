//! Applet implementations for grep, cat, tail, head, wc, sha256.

use crate::vfs::SafeVfs;
use std::path::Path;

/// POSIX-style grep runner.
pub fn run_grep(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err(
            "Usage: grep [-r|-rn|-i] <pattern> [file...] [--include=<pattern>]".to_string(),
        );
    }

    let mut recursive = false;
    let mut show_lines = false;
    let mut ignore_case = false;
    let mut include_pattern: Option<String> = None;
    let mut pattern: Option<String> = None;
    let mut files = Vec::new();

    for arg in args {
        if arg.starts_with("--include=") {
            include_pattern = Some(arg.trim_start_matches("--include=").to_string());
        } else if arg == "-rn" || arg == "-nr" {
            recursive = true;
            show_lines = true;
        } else if arg == "-r" {
            recursive = true;
        } else if arg == "-n" {
            show_lines = true;
        } else if arg == "-i" {
            ignore_case = true;
        } else if pattern.is_none() {
            pattern = Some(arg.clone());
        } else {
            files.push(arg.clone());
        }
    }

    let query = pattern.ok_or_else(|| "grep: missing search pattern".to_string())?;
    let search_query = if ignore_case {
        query.to_lowercase()
    } else {
        query.clone()
    };

    let mut output = String::new();

    if recursive {
        let root = if files.is_empty() {
            Path::new(".")
        } else {
            Path::new(&files[0])
        };
        let matched_paths = SafeVfs::find_files(root, include_pattern.as_deref());

        for path in matched_paths {
            if let Ok(content) = std::fs::read_to_string(&path) {
                for (idx, line) in content.lines().enumerate() {
                    let hay = if ignore_case {
                        line.to_lowercase()
                    } else {
                        line.to_string()
                    };
                    if hay.contains(&search_query) {
                        if show_lines {
                            output.push_str(&format!("{}:{}:{}\n", path.display(), idx + 1, line));
                        } else {
                            output.push_str(&format!("{}:{}\n", path.display(), line));
                        }
                    }
                }
            }
        }
    } else {
        if files.is_empty() {
            return Err("grep: missing target file".to_string());
        }
        for file in files {
            let content = SafeVfs::read_to_string(&file)?;
            for (idx, line) in content.lines().enumerate() {
                let hay = if ignore_case {
                    line.to_lowercase()
                } else {
                    line.to_string()
                };
                if hay.contains(&search_query) {
                    if show_lines {
                        output.push_str(&format!("{}:{}:{}\n", file, idx + 1, line));
                    } else {
                        output.push_str(&format!("{}:{}\n", file, line));
                    }
                }
            }
        }
    }

    Ok(output)
}

/// POSIX cat runner.
pub fn run_cat(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: cat [-n] <file>".to_string());
    }

    let mut show_lines = false;
    let mut file: Option<&str> = None;

    for arg in args {
        if arg == "-n" {
            show_lines = true;
        } else if file.is_none() {
            file = Some(arg);
        }
    }

    let target = file.ok_or_else(|| "cat: missing file argument".to_string())?;
    let content = SafeVfs::read_to_string(target)?;

    if show_lines {
        let mut out = String::new();
        for (idx, line) in content.lines().enumerate() {
            out.push_str(&format!("{:6}\t{}\n", idx + 1, line));
        }
        Ok(out)
    } else {
        Ok(content)
    }
}

/// POSIX head runner.
pub fn run_head(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: head [-n <lines>] <file>".to_string());
    }
    let mut limit: usize = 10;
    let mut file: Option<&str> = None;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "-n" && i + 1 < args.len() {
            limit = args[i + 1].parse().unwrap_or(10);
            i += 2;
        } else {
            file = Some(&args[i]);
            i += 1;
        }
    }

    let target = file.ok_or_else(|| "head: missing file argument".to_string())?;
    let content = SafeVfs::read_to_string(target)?;
    let lines: Vec<&str> = content.lines().take(limit).collect();
    Ok(lines.join("\n"))
}

/// POSIX tail runner.
pub fn run_tail(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: tail [-n <lines>] <file>".to_string());
    }
    let mut limit: usize = 10;
    let mut file: Option<&str> = None;

    let mut i = 0;
    while i < args.len() {
        if args[i] == "-n" && i + 1 < args.len() {
            limit = args[i + 1].parse().unwrap_or(10);
            i += 2;
        } else {
            file = Some(&args[i]);
            i += 1;
        }
    }

    let target = file.ok_or_else(|| "tail: missing file argument".to_string())?;
    let content = SafeVfs::read_to_string(target)?;
    let all_lines: Vec<&str> = content.lines().collect();
    let start = all_lines.len().saturating_sub(limit);
    Ok(all_lines[start..].join("\n"))
}

/// POSIX wc (word, line, byte count).
pub fn run_wc(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: wc [-l] <file>".to_string());
    }
    let file = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .ok_or_else(|| "wc: missing file argument".to_string())?;
    let content = SafeVfs::read_to_string(file)?;

    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let bytes = content.len();

    if args.contains(&"-l".to_string()) {
        Ok(format!("{} {}", lines, file))
    } else {
        Ok(format!("{} {} {} {}", lines, words, bytes, file))
    }
}

/// sha256 checksum calculator (clean FFI / SHA256 without heavy native crates).
pub fn run_sha256sum(args: &[String]) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: sha256sum <file>".to_string());
    }
    let file = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .ok_or_else(|| "sha256sum: missing file argument".to_string())?;
    let bytes = SafeVfs::read_bytes(file)?;

    // Lightweight portable SHA-256 computation
    let hash = simple_sha256(&bytes);
    Ok(format!("{}  {}", hash, file))
}

#[allow(clippy::chunks_exact_to_as_chunks)]
fn simple_sha256(data: &[u8]) -> String {
    // Standard SHA-256 implementation
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut msg = data.to_vec();
    let bit_len = (data.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut h_val = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h_val
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h_val = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(h_val);
    }

    h.iter().map(|val| format!("{:08x}", val)).collect()
}

/// POSIX ls runner.
pub fn run_ls(args: &[String]) -> Result<String, String> {
    let mut show_all = false;
    let mut long_format = false;
    let mut dir: Option<&str> = None;

    for arg in args {
        if arg.starts_with('-') {
            if arg.contains('a') {
                show_all = true;
            }
            if arg.contains('l') {
                long_format = true;
            }
        } else if dir.is_none() {
            dir = Some(arg);
        }
    }

    let target_str = dir.unwrap_or(".");
    let target_path = if target_str == "." {
        crate::vfs::SafeVfs::resolve_path("configs").and_then(|p| p.parent().map(|p| p.to_path_buf())).unwrap_or_else(|| std::path::PathBuf::from("."))
    } else {
        crate::vfs::SafeVfs::resolve_path(target_str)
            .ok_or_else(|| format!("Directory not found: {target_str}"))?
    };

    let entries = std::fs::read_dir(&target_path)
        .map_err(|e| format!("Failed to read directory '{target_str}': {e}"))?;

    let mut names = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !show_all && name.starts_with('.') {
            continue;
        }

        if long_format {
            let metadata = entry.metadata().ok();
            let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            let kind = if is_dir { "d" } else { "-" };
            names.push(format!("{kind}rwxr-xr-x {:8} {}", size, name));
        } else {
            names.push(name);
        }
    }

    names.sort();
    let mut out = names.join(if long_format { "\n" } else { "  " });
    out.push('\n');
    Ok(out)
}
