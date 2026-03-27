mod buffer;
mod lexer;
mod report;
mod token;
mod utils;

use clap::Parser;
use memmap2::Mmap;
use std::{fs::File, path::Path};

use crate::{lexer::Lexer, report::fatal};

/// The `Ima` language compiler
#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    /// Source program
    #[arg(required = true)]
    #[arg(value_name = "SOURCE")]
    #[arg(num_args = 1..)]
    #[arg(value_hint = clap::ValueHint::FilePath)]
    sources: Vec<String>,
}

fn main() {
    let args = Args::parse();
    for source_path in &args.sources {
        let path = Path::new(source_path);
        if !path.exists() {
            panic!("Source path: '{}', does not exist.", source_path);
        }
        if !path.is_file() {
            panic!("Source path: '{}', is not a file", source_path);
        }

        let source_file = File::open(path).unwrap_or_else(|err| {
            panic!("Cannot open file '{}': {}", source_path, err);
        });

        match unsafe { Mmap::map(&source_file) } {
            Ok(mmap) => {
                let lexer = Lexer::new(source_path, &mmap[..]);
                for token in lexer {
                    println!("{}", token);
                }
            }
            Err(err) => fatal(&err.to_string()),
        }
    }
}
