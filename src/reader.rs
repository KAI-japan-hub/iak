//! ディレクトリの読み込み処理

use crate::entry::{FileEntry, FileKind, LongInfo, SortKey};
use chrono::{DateTime, Local};
use ignore::WalkBuilder;
use std::fs;
use std::os::unix::fs::MetadataExt;
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

/// メタデータからLongInfoを生成する
fn build_long_info(meta: &fs::Metadata) -> LongInfo {
    LongInfo {
        mode: meta.mode(),
        nlink: meta.nlink(),
        uid: meta.uid(),
        gid: meta.gid(),
    }
}

/// std::fs::DirEntryからFileEntryを生成する（.gitignoreを考慮しない通常経路）
fn build_file_entry_plain(dir_entry: &fs::DirEntry, want_long: bool) -> std::io::Result<FileEntry> {
    let meta = dir_entry.metadata()?;
    let name = dir_entry.file_name().to_string_lossy().to_string();
    let kind = detect_kind(&name, meta.is_dir());
    let modified: DateTime<Local> = meta.modified()?.into();
    let long_info = if want_long {
        Some(build_long_info(&meta))
    } else {
        None
    };
    Ok(FileEntry {
        path: dir_entry.path(),
        name,
        kind,
        size: meta.len(),
        modified,
        long_info,
    })
}

/// .gitignoreを尊重してディレクトリのエントリ一覧を読み込む
fn read_entries_respecting_ignore(
    dir: &Path,
    want_long: bool,
) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    let walker = WalkBuilder::new(dir).max_depth(Some(1)).build();
    for result in walker {
        let dir_entry = match result {
            Ok(e) => e,
            Err(_) => continue,
        };
        // ルート自身（max_depth=1の起点）はスキップする
        if dir_entry.path() == dir {
            continue;
        }
        let meta = match dir_entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        let name = dir_entry
            .path()
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let kind = detect_kind(&name, meta.is_dir());
        let modified: DateTime<Local> = match meta.modified() {
            Ok(m) => m.into(),
            Err(_) => continue,
        };
        let long_info = if want_long {
            Some(build_long_info(&meta))
        } else {
            None
        };
        entries.push(FileEntry {
            path: dir_entry.path().to_path_buf(),
            name,
            kind,
            size: meta.len(),
            modified,
            long_info,
        });
    }
    Ok(entries)
}

/// ディレクトリのエントリ一覧を読み込んで指定順に並べて返す
pub fn read_entries(
    dir: &Path,
    sort_key: SortKey,
    respect_ignore: bool,
    want_long: bool,
) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = if respect_ignore {
        read_entries_respecting_ignore(dir, want_long)?
    } else {
        let mut v = Vec::new();
        for dir_entry in fs::read_dir(dir)? {
            let entry = build_file_entry_plain(&dir_entry?, want_long)?;
            v.push(entry);
        }
        v
    };
    match sort_key {
        SortKey::Name => entries.sort_by(|a, b| a.name.cmp(&b.name)),
        SortKey::Size => entries.sort_by(|a, b| a.size.cmp(&b.size)),
        SortKey::Mtime => entries.sort_by(|a, b| a.modified.cmp(&b.modified)),
    }
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
