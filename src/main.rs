use std::path::Path;

use clap::Parser;
use lib::{Scanner, Tokenizer};

#[derive(Parser, Debug, Clone)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let file = args.file;
    let scanner = Scanner::from_path(Path::new(&file)).expect("where file");
    let tokenizer = Tokenizer::new(scanner);
    tokenizer.for_each(|t| println!("{:?}", t));
}
