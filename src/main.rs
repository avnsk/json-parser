use std::{fs, process, path::PathBuf};

use clap::Parser;


#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}
fn main() {
    let args = Args::parse();
    let content = match fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            eprint!("Error reading file {}, {}", args.file.display(), e);
            process::exit(1);
        }
    };
    println!("{:?}", content);
}
