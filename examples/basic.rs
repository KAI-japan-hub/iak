//! 基本的な使用例：カレントディレクトリのファイル一覧を表示する

use iak::display::DisplayOptions;
use iak::entry::SortKey;
use iak::{display, reader};
use std::path::Path;

fn main() {
    let dir = Path::new(".");
    let opts = DisplayOptions {
        new_mark: true,
        humanize: true,
        long: false,
    };
    match reader::read_entries(dir, SortKey::Name, false, false) {
        Ok(entries) => {
            for line in display::format_entries(&entries, &opts) {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
