//! GoldSrc.rs Standard Core Utilities Suite (`goldsrc:coreutils`).
//!
//! Exposes canonical POSIX utilities (`grep`, `cat`, `tail`, `head`, `wc`, `sha256sum`)
//! directly into the dedicated server console with strict sandboxed VFS protection.

pub mod applets;
pub mod vfs;

use goldsrc::prelude::*;

/// Tokenizes command line arguments respecting double quotes.
fn parse_shell_args(input: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;

    for ch in input.chars() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            c if c.is_whitespace() && !in_quotes => {
                if !current.is_empty() {
                    args.push(current.clone());
                    current.clear();
                }
            }
            c => {
                current.push(c);
            }
        }
    }
    if !current.is_empty() {
        args.push(current);
    }
    args
}

pub struct Coreutils;

#[plugin(
    name = "coreutils",
    role = "service",
    bundle = "server",
    version = "0.20.0",
    author = "GoldSrc.rs Team",
    description = "Canonical POSIX server utilities suite (grep, cat, tail, head, wc, sha256sum, ls)",
    url = "https://github.com/goldsrc-rs/goldsrc-plugins-standard"
)]
impl Coreutils {
    #[on_load]
    fn init() {
        log_info!("[Coreutils] Initializing server utilities suite (v0.20.0)...");
    }

    #[command(
        name = "grep",
        description = "Search files for pattern matching POSIX grep",
        usage = "grep [-r|-rn|-i] <pattern> [file...] [--include=<pattern>]"
    )]
    fn cmd_grep(args: String) {
        let raw_args = parse_shell_args(&args);
        match applets::run_grep(&raw_args) {
            Ok(output) => {
                Player::new(0).print_console(output);
            }
            Err(err) => Player::new(0).print_console(format!("grep: {err}\n")),
        }
    }

    #[command(
        name = "cat",
        description = "Concatenate and print file contents",
        usage = "cat [-n] <file>"
    )]
    fn cmd_cat(args: String) {
        let raw_args = parse_shell_args(&args);
        match applets::run_cat(&raw_args) {
            Ok(output) => {
                Player::new(0).print_console(output);
            }
            Err(err) => Player::new(0).print_console(format!("cat: {err}\n")),
        }
    }

    #[command(
        name = "head",
        description = "Output first part of files",
        usage = "head [-n <lines>] <file>"
    )]
    fn cmd_head(args: String) {
        let raw_args = parse_shell_args(&args);
        match applets::run_head(&raw_args) {
            Ok(output) => {
                Player::new(0).print_console(output);
            }
            Err(err) => Player::new(0).print_console(format!("head: {err}\n")),
        }
    }

    #[command(
        name = "tail",
        description = "Output last part of files",
        usage = "tail [-n <lines>] <file>"
    )]
    fn cmd_tail(args: String) {
        let raw_args = parse_shell_args(&args);
        match applets::run_tail(&raw_args) {
            Ok(output) => {
                Player::new(0).print_console(output);
            }
            Err(err) => Player::new(0).print_console(format!("tail: {err}\n")),
        }
    }

    #[command(
        name = "wc",
        description = "Print newline, word, and byte counts",
        usage = "wc [-l] <file>"
    )]
    fn cmd_wc(args: String) {
        let raw_args = parse_shell_args(&args);
        match applets::run_wc(&raw_args) {
            Ok(output) => Player::new(0).print_console(format!("{output}\n")),
            Err(err) => Player::new(0).print_console(format!("wc: {err}\n")),
        }
    }

    #[command(
        name = "sha256sum",
        description = "Compute SHA256 checksum of files",
        usage = "sha256sum <file>"
    )]
    fn cmd_sha256sum(args: String) {
        let raw_args = parse_shell_args(&args);
        match applets::run_sha256sum(&raw_args) {
            Ok(output) => Player::new(0).print_console(format!("{output}\n")),
            Err(err) => Player::new(0).print_console(format!("sha256sum: {err}\n")),
        }
    }

    #[command(
        name = "ls",
        description = "List directory contents",
        usage = "ls [-la] [dir]"
    )]
    fn cmd_ls(args: String) {
        let raw_args = parse_shell_args(&args);
        match applets::run_ls(&raw_args) {
            Ok(output) => Player::new(0).print_console(output),
            Err(err) => Player::new(0).print_console(format!("ls: {err}\n")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grep_simple() {
        let temp_file = std::env::temp_dir().join(format!("test_grep_{}.txt", std::process::id()));
        std::fs::write(&temp_file, "hello world\nfoo bar baz\nrust goldsrc\n").unwrap();

        let args = vec!["foo".to_string(), temp_file.to_str().unwrap().to_string()];
        let out = applets::run_grep(&args).unwrap();
        assert!(out.contains("foo bar baz"));

        let _ = std::fs::remove_file(temp_file);
    }

    #[test]
    fn test_head_and_tail() {
        let temp_file = std::env::temp_dir().join(format!("test_ht_{}.txt", std::process::id()));
        std::fs::write(&temp_file, "1\n2\n3\n4\n5\n").unwrap();

        let path_str = temp_file.to_str().unwrap().to_string();

        let head_args = vec!["-n".to_string(), "2".to_string(), path_str.clone()];
        let head_out = applets::run_head(&head_args).unwrap();
        assert_eq!(head_out, "1\n2");

        let tail_args = vec!["-n".to_string(), "2".to_string(), path_str];
        let tail_out = applets::run_tail(&tail_args).unwrap();
        assert_eq!(tail_out, "4\n5");

        let _ = std::fs::remove_file(temp_file);
    }
}
