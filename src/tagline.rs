//! README.mdの要約表示処理

use std::fs;
use std::path::Path;

/// 指定ディレクトリのREADME.mdから最初のタイトル行を除いた
/// 最初の非空行（概要説明）を取り出す
pub fn extract_tagline(dir: &Path) -> Option<String> {
    let readme_path = dir.join("README.md");
    let content = fs::read_to_string(readme_path).ok()?;
    let mut lines = content.lines();
    // 最初の "# タイトル" 行をスキップする
    for line in &mut lines {
        if line.trim_start().starts_with('#') {
            break;
        }
    }
    // その後の最初の空でない行を概要として返す
    for line in lines {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            return Some(trimmed.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_extract_tagline_basic() {
        let dir = std::env::temp_dir().join("iak_tagline_test");
        fs::create_dir_all(&dir).unwrap();
        let mut f = fs::File::create(dir.join("README.md")).unwrap();
        writeln!(f, "# iak").unwrap();
        writeln!(f).unwrap();
        writeln!(f, "A color-coded ls for clearer file display.").unwrap();
        let result = extract_tagline(&dir);
        assert_eq!(
            result.as_deref(),
            Some("A color-coded ls for clearer file display.")
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_extract_tagline_missing_file() {
        let dir = std::env::temp_dir().join("iak_tagline_missing_test");
        fs::create_dir_all(&dir).unwrap();
        let result = extract_tagline(&dir);
        assert_eq!(result, None);
        fs::remove_dir_all(&dir).ok();
    }
}
