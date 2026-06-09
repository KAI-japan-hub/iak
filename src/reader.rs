//! ディレクトリの読み込み処理

use crate::entry::{FileEntry, FileKind};
use chrono::{DateTime, Local};
use std::fs;
use std::path::Path;

/// ファイル名の拡張子からFileKindを判定する
pub fn detect_kind(name: &str, is_dir: bool) -> FileKind {
    if is_dir {
        return FileKind::Directory;
    }
    match name.rsplit('.').next().unwrap_or("") {
        "md" => FileKind::Markdown,
        "pdf" => FileKind::Pdf,
        "rs" => FileKind::Rust,
        "toml" => FileKind::Toml,
        _ => FileKind::Other,
    }
}

/// DirEntryからFileEntryを生成する
fn build_file_entry(dir_entry: &fs::DirEntry) -> std::io::Result<FileEntry> {
    let meta = dir_entry.metadata()?;
    let name = dir_entry.file_name().to_string_lossy().to_string();
    let kind = detect_kind(&name, meta.is_dir());
    let modified: DateTime<Local> = meta.modified()?.into();
    Ok(FileEntry {
        path: dir_entry.path(),
        name,
        kind,
        size: meta.len(),
        modified,
    })
}

/// ディレクトリのエントリ一覧を読み込んで名前順に返す
pub fn read_entries(dir: &Path) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    for dir_entry in fs::read_dir(dir)? {
        let entry = build_file_entry(&dir_entry?)?;
        entries.push(entry);
    }
    entries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_kind_directory() {
        assert_eq!(detect_kind("src", true), FileKind::Directory);
    }

    #[test]
    fn test_detect_kind_markdown() {
        assert_eq!(detect_kind("README.md", false), FileKind::Markdown);
    }

    #[test]
    fn test_detect_kind_pdf() {
        assert_eq!(detect_kind("report.pdf", false), FileKind::Pdf);
    }

    #[test]
    fn test_detect_kind_rust() {
        assert_eq!(detect_kind("main.rs", false), FileKind::Rust);
    }

    #[test]
    fn test_detect_kind_toml() {
        assert_eq!(detect_kind("Cargo.toml", false), FileKind::Toml);
    }

    #[test]
    fn test_detect_kind_other() {
        assert_eq!(detect_kind("image.png", false), FileKind::Other);
    }
}
