//! iak CLI エントリポイント

mod args;
mod gencomp;

use args::Args;
use clap::Parser;
use iak::{display, reader, tagline};
use std::path::{Path, PathBuf};

fn main() {
    let args = Args::parse();

    if let Some(outdir) = &args.completions {
        gencomp::generate_all(outdir);
        return;
    }

    let dir: PathBuf = args
        .file
        .clone()
        .unwrap_or_else(|| Path::new(".").to_path_buf());

    if args.tagline {
        match tagline::extract_tagline(&dir) {
            Some(line) => println!("{}", line),
            None => eprintln!("Error: README.md not found or has no summary line"),
        }
        return;
    }

    let sort_key = args.sort.into();
    let opts = display::DisplayOptions {
        new_mark: args.new_mark,
        humanize: args.humanize,
        long: args.long,
    };

    match reader::read_entries(&dir, sort_key, args.respect_ignore, args.long) {
        Ok(entries) => {
            for line in display::format_entries(&entries, &opts) {
                println!("{}", line);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
