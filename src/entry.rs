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

/// -l/--long 指定時にのみ取得する詳細情報
#[derive(Debug, Clone, Copy)]
pub struct LongInfo {
    /// パーミッションのモードビット（例: 0o755）
    pub mode: u32,
    /// ハードリンク数
    pub nlink: u64,
    /// 所有者のユーザーID
    pub uid: u32,
    /// 所有者のグループID
    pub gid: u32,
}

/// ディレクトリエントリの情報
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub kind: FileKind,
    pub size: u64,
    pub modified: DateTime<Local>,
    pub long_info: Option<LongInfo>,
}
