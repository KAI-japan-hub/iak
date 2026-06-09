//! iak CLI エントリポイント

use iak::{display, reader};
use std::path::Path;

fn main() {
    let dir = Path::new(".");
    match reader::read_entries(dir) {
        Ok(entries) => {
            for line in display::format_entries(&entries) {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
