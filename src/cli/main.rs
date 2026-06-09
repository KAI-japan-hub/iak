//! iak CLI エントリポイント

use std::path::Path;
use iak::{display, reader};

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
