//! ファイルエントリの表示処理

use crate::entry::{FileEntry, FileKind};
use chrono::Local;
use colored::Colorize;

/// FileKindに応じた色付きのファイル名文字列を返す
fn colorize_name(entry: &FileEntry) -> String {
    match entry.kind {
        FileKind::Directory => entry.name.bold().blue().to_string(),
        FileKind::Markdown => entry.name.bold().green().to_string(),
        FileKind::Pdf => entry.name.red().to_string(),
        FileKind::Rust => entry.name.yellow().to_string(),
        FileKind::Toml => entry.name.cyan().to_string(),
        FileKind::Other => entry.name.normal().to_string(),
    }
}

/// 更新日時が24時間以内かどうかを判定する
fn is_recently_modified(entry: &FileEntry) -> bool {
    let now = Local::now();
    let diff = now.signed_duration_since(entry.modified);
    diff.num_hours() < 24
}

/// ファイルサイズを見やすい単位の文字列に変換する
fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{:>6}B", size)
    } else if size < 1024 * 1024 {
        format!("{:>5.1}K", size as f64 / 1024.0)
    } else {
        format!("{:>5.1}M", size as f64 / (1024.0 * 1024.0))
    }
}

/// 1件のエントリを1行の文字列として返す
pub fn format_entry(entry: &FileEntry) -> String {
    let recent_mark = if is_recently_modified(entry) {
        "★"
    } else {
        "  "
    };
    let size_str = format_size(entry.size);
    let date_str = entry.modified.format("%Y-%m-%d %H:%M").to_string();
    let name_str = colorize_name(entry);
    format!("{} {} {}  {}", recent_mark, size_str, date_str, name_str)
}

/// エントリ一覧を表示用の文字列リストに変換して返す
pub fn format_entries(entries: &[FileEntry]) -> Vec<String> {
    entries.iter().map(format_entry).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    use std::path::PathBuf;

    fn make_entry(name: &str, kind: FileKind, size: u64) -> FileEntry {
        FileEntry {
            path: PathBuf::from(name),
            name: name.to_string(),
            kind,
            size,
            modified: Local::now(),
        }
    }

    #[test]
    fn test_format_size_bytes() {
        let entry = make_entry("file.txt", FileKind::Other, 512);
        let result = format_entry(&entry);
        assert!(result.contains("512B"));
    }

    #[test]
    fn test_format_size_kilobytes() {
        let entry = make_entry("file.txt", FileKind::Other, 2048);
        let result = format_entry(&entry);
        assert!(result.contains("K"));
    }

    #[test]
    fn test_format_size_megabytes() {
        let entry = make_entry("file.txt", FileKind::Other, 2 * 1024 * 1024);
        let result = format_entry(&entry);
        assert!(result.contains("M"));
    }

    #[test]
    fn test_format_entry_contains_name() {
        let entry = make_entry("README.md", FileKind::Markdown, 100);
        let result = format_entry(&entry);
        assert!(result.contains("README.md"));
    }

    #[test]
    fn test_format_entries_count() {
        let entries = vec![
            make_entry("a.rs", FileKind::Rust, 100),
            make_entry("b.md", FileKind::Markdown, 200),
        ];
        let lines = format_entries(&entries);
        assert_eq!(lines.len(), 2);
    }
}
