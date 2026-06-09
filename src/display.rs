//! ファイルエントリの表示処理

use colored::Colorize;
use chrono::Local;
use crate::entry::{FileEntry, FileKind};

/// FileKindに応じた色付きのファイル名文字列を返す
fn colorize_name(entry: &FileEntry) -> String {
    match entry.kind {
        FileKind::Directory => entry.name.bold().blue().to_string(),
        FileKind::Markdown  => entry.name.bold().green().to_string(),
        FileKind::Pdf       => entry.name.red().to_string(),
        FileKind::Rust      => entry.name.yellow().to_string(),
        FileKind::Toml      => entry.name.cyan().to_string(),
        FileKind::Other     => entry.name.normal().to_string(),
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
    let recent_mark = if is_recently_modified(entry) { "★" } else { "  " };
    let size_str = format_size(entry.size);
    let date_str = entry.modified.format("%Y-%m-%d %H:%M").to_string();
    let name_str = colorize_name(entry);
    format!("{} {} {}  {}", recent_mark, size_str, date_str, name_str)
}

/// エントリ一覧を表示用の文字列リストに変換して返す
pub fn format_entries(entries: &[FileEntry]) -> Vec<String> {
    entries.iter().map(format_entry).collect()
}
