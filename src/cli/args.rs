//! コマンドライン引数の定義

use clap::{Parser, ValueEnum};
use iak::entry::SortKey;
use std::path::PathBuf;

/// ソート順の指定（CLI用）
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum SortKeyArg {
    Name,
    Size,
    Mtime,
}

impl From<SortKeyArg> for SortKey {
    fn from(value: SortKeyArg) -> Self {
        match value {
            SortKeyArg::Name => SortKey::Name,
            SortKeyArg::Size => SortKey::Size,
            SortKeyArg::Mtime => SortKey::Mtime,
        }
    }
}

/// A color-coded ls for clearer file display.
#[derive(Parser, Debug)]
#[command(name = "iak", version, about, long_about = None)]
pub struct Args {
    /// Path of the file or directory to display.
    /// If omitted, the current directory is shown.
    pub file: Option<PathBuf>,

    /// Show detailed file information
    #[arg(short = 'l', long)]
    pub long: bool,

    /// Mark files created or updated within 24 hours as "new"
    #[arg(long)]
    pub new_mark: bool,

    /// Display file sizes in a human-readable format
    #[arg(long)]
    pub humanize: bool,

    /// Show a summary of README.md
    #[arg(long)]
    pub tagline: bool,

    /// Show the title inside a PDF file
    #[arg(long)]
    pub pdf_title: bool,

    /// Respect rules such as .gitignore
    #[arg(long)]
    pub respect_ignore: bool,

    /// Specify the sort order [name | size | mtime]
    #[arg(long, value_enum, default_value_t = SortKeyArg::Name)]
    pub sort: SortKeyArg,

    /// Generate shell completion files into the given directory
    #[arg(long, value_name = "DIR")]
    pub completions: Option<PathBuf>,
}
