//! 結合テスト：ライブラリの外側からの動作確認

use std::path::Path;
use iak::{display, reader};

#[test]
fn test_read_entries_returns_results() {
    let dir = Path::new(".");
    let entries = reader::read_entries(dir).expect("ディレクトリの読み込みに失敗");
    assert!(!entries.is_empty());
}

#[test]
fn test_format_entries_returns_same_count() {
    let dir = Path::new(".");
    let entries = reader::read_entries(dir).expect("ディレクトリの読み込みに失敗");
    let lines = display::format_entries(&entries);
    assert_eq!(entries.len(), lines.len());
}

#[test]
fn test_each_line_contains_filename() {
    let dir = Path::new(".");
    let entries = reader::read_entries(dir).expect("ディレクトリの読み込みに失敗");
    let lines = display::format_entries(&entries);
    for (entry, line) in entries.iter().zip(lines.iter()) {
        assert!(line.contains(&entry.name));
    }
}
