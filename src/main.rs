use std::{fs, path::PathBuf, process};

use clap::Parser;
use json_parser::{JsonParser, lex};
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
    let tokens = match lex(&content) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Invalid JSON: {}", e);
            process::exit(1);
        }
    };
    let mut parser = JsonParser::new(&tokens);
    match parser.parse() {
        Ok(_) => {
            println!("Valid JSON");
            process::exit(0);
        }
        Err(err) => {
            eprintln!("Invalid JSON: {}", err);
            process::exit(1);
        }
    }
}
