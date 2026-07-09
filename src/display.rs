//! ファイルエントリの表示処理

use crate::entry::{FileEntry, FileKind, LongInfo};
use chrono::Local;
use colored::Colorize;

/// 表示オプション
#[derive(Debug, Clone, Copy, Default)]
pub struct DisplayOptions {
    /// 24時間以内に更新されたファイルに★マークを付けるか
    pub new_mark: bool,
    /// ファイルサイズを人間可読な単位（K/M）で表示するか
    pub humanize: bool,
    /// 詳細情報（パーミッション・リンク数・UID/GID）を表示するか
    pub long: bool,
}

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

/// ファイルサイズを人間可読な単位の文字列に変換する
fn format_size_human(size: u64) -> String {
    if size < 1024 {
        format!("{:>6}B", size)
    } else if size < 1024 * 1024 {
        format!("{:>5.1}K", size as f64 / 1024.0)
    } else {
        format!("{:>5.1}M", size as f64 / (1024.0 * 1024.0))
    }
}

/// ファイルサイズを生バイト数の文字列に変換する
fn format_size_raw(size: u64) -> String {
    format!("{:>7}B", size)
}

fn format_size(size: u64, humanize: bool) -> String {
    if humanize {
        format_size_human(size)
    } else {
        format_size_raw(size)
    }
}

/// パーミッションのモードビットを rwxr-xr-x 形式の文字列に変換する
fn format_mode(mode: u32) -> String {
    let bits = [
        (0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
        (0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
        (0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
    ];
    bits.iter()
        .map(|(bit, ch)| if mode & bit != 0 { *ch } else { '-' })
        .collect()
}

/// -l/--long 指定時の詳細情報部分を整形する
fn format_long_info(info: &LongInfo) -> String {
    format!(
        "{} {:>3} {:>5}:{:<5}",
        format_mode(info.mode),
        info.nlink,
        info.uid,
        info.gid
    )
}

/// 1件のエントリを1行の文字列として返す
pub fn format_entry(entry: &FileEntry, opts: &DisplayOptions) -> String {
    let recent_mark = if opts.new_mark && is_recently_modified(entry) {
        "★"
    } else {
        "  "
    };
    let size_str = format_size(entry.size, opts.humanize);
    let date_str = entry.modified.format("%Y-%m-%d %H:%M").to_string();
    let name_str = colorize_name(entry);

    if opts.long {
        if let Some(info) = &entry.long_info {
            let long_str = format_long_info(info);
            return format!(
                "{} {} {} {}  {}",
                recent_mark, long_str, size_str, date_str, name_str
            );
        }
    }
    format!("{} {} {}  {}", recent_mark, size_str, date_str, name_str)
}

/// エントリ一覧を表示用の文字列リストに変換して返す
pub fn format_entries(entries: &[FileEntry], opts: &DisplayOptions) -> Vec<String> {
    entries.iter().map(|e| format_entry(e, opts)).collect()
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
            long_info: None,
        }
    }

    fn humanize_opts() -> DisplayOptions {
        DisplayOptions {
            new_mark: false,
            humanize: true,
            long: false,
        }
    }

    #[test]
    fn test_format_size_bytes() {
        let entry = make_entry("file.txt", FileKind::Other, 512);
        let result = format_entry(&entry, &humanize_opts());
        assert!(result.contains("512B"));
    }

    #[test]
    fn test_format_size_kilobytes() {
        let entry = make_entry("file.txt", FileKind::Other, 2048);
        let result = format_entry(&entry, &humanize_opts());
        assert!(result.contains("K"));
    }

    #[test]
    fn test_format_size_megabytes() {
        let entry = make_entry("file.txt", FileKind::Other, 2 * 1024 * 1024);
        let result = format_entry(&entry, &humanize_opts());
        assert!(result.contains("M"));
    }

    #[test]
    fn test_format_entry_contains_name() {
        let entry = make_entry("README.md", FileKind::Markdown, 100);
        let result = format_entry(&entry, &humanize_opts());
        assert!(result.contains("README.md"));
    }

    #[test]
    fn test_format_entries_count() {
        let entries = vec![
            make_entry("a.rs", FileKind::Rust, 100),
            make_entry("b.md", FileKind::Markdown, 200),
        ];
        let lines = format_entries(&entries, &humanize_opts());
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn test_format_size_raw_default() {
        let entry = make_entry("file.txt", FileKind::Other, 2048);
        let opts = DisplayOptions {
            new_mark: false,
            humanize: false,
            long: false,
        };
        let result = format_entry(&entry, &opts);
        assert!(result.contains("2048B"));
    }

    #[test]
    fn test_new_mark_off_by_default() {
        let entry = make_entry("file.txt", FileKind::Other, 100);
        let opts = DisplayOptions {
            new_mark: false,
            humanize: true,
            long: false,
        };
        let result = format_entry(&entry, &opts);
        assert!(!result.contains('★'));
    }

    #[test]
    fn test_format_mode_rwxr_xr_x() {
        assert_eq!(format_mode(0o755), "rwxr-xr-x");
    }

    #[test]
    fn test_format_mode_rw_r_r() {
        assert_eq!(format_mode(0o644), "rw-r--r--");
    }

    #[test]
    fn test_long_info_included_when_present() {
        let mut entry = make_entry("file.txt", FileKind::Other, 100);
        entry.long_info = Some(LongInfo {
            mode: 0o644,
            nlink: 1,
            uid: 501,
            gid: 20,
        });
        let opts = DisplayOptions {
            new_mark: false,
            humanize: true,
            long: true,
        };
        let result = format_entry(&entry, &opts);
        assert!(result.contains("rw-r--r--"));
        assert!(result.contains("501"));
    }
}
