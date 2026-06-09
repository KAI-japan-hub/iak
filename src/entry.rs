//! ファイルエントリの定義

use std::path::PathBuf;
use chrono::{DateTime, Local};

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

/// ディレクトリエントリの情報
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileKind,
    pub size: u64,
    pub modified: DateTime<Local>,
}
