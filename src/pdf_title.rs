//! PDFファイル内のタイトル抽出処理

use lopdf::Document;
use std::path::Path;

/// 指定されたPDFファイルのInfo辞書からタイトルを抽出する
pub fn extract_pdf_title(path: &Path) -> Option<String> {
    let doc = Document::load(path).ok()?;
    let info_ref = doc.trailer.get(b"Info").ok()?;
    let info_dict = doc.get_dictionary(info_ref.as_reference().ok()?).ok()?;
    let title_obj = info_dict.get(b"Title").ok()?;
    let title_bytes = title_obj.as_str().ok()?;
    Some(decode_pdf_string(title_bytes))
}

/// PDF文字列（UTF-16BE with BOM、もしくはPDFDocEncoding）をUTF-8文字列に変換する
fn decode_pdf_string(bytes: &[u8]) -> String {
    if bytes.len() >= 2 && bytes[0] == 0xFE && bytes[1] == 0xFF {
        // UTF-16BE (BOM付き)
        let utf16_units: Vec<u16> = bytes[2..]
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();
        String::from_utf16_lossy(&utf16_units)
    } else {
        // PDFDocEncoding は ASCII 範囲においては概ね Latin-1 と同一とみなす
        bytes.iter().map(|&b| b as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_pdf_string_ascii() {
        let result = decode_pdf_string(b"Hello");
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_decode_pdf_string_utf16_bom() {
        // "Hi" in UTF-16BE with BOM
        let bytes = [0xFE, 0xFF, 0x00, 0x48, 0x00, 0x69];
        let result = decode_pdf_string(&bytes);
        assert_eq!(result, "Hi");
    }

    #[test]
    fn test_extract_pdf_title_missing_file() {
        let path = Path::new("/tmp/definitely_does_not_exist_iak_test.pdf");
        let result = extract_pdf_title(path);
        assert_eq!(result, None);
    }
}
