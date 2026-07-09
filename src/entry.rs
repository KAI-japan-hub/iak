//! ファイルエントリの定義

use chrono::{DateTime, Local};
use std::path::PathBuf;

/// ファイルの種別
#[derive(Debug, Clone, PartialEq)]
pub enum FileKind {
    Directory,
    Markdown,
    Pdf,
    Rust,
    Toml,
    Other,
}

/// ソート順の指定
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Name,
    Size,
    Mtime,
}

/// ディレクトリエントリの情報
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileKind,
    pub size: u64,
    pub modified: DateTime<Local>,
}
