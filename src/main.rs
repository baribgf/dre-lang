mod ast;
mod buffer;
mod lexer;
mod parser;
mod report;
mod token;
mod utils;

use std::{fs::File, path::Path};

use clap::Parser as ArgParser;
use lalrpop_util::ParseError;
use memmap2::Mmap;
use std::{fs::File, path::Path};

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::report::{error, fatal};
use crate::utils::get_nth_line;

/// The `Dre` language compiler
#[derive(ArgParser)]
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
                let source = &mmap[..];
                let lexer = Lexer::new(source_path, source);
                /* for token in lexer {
                    println!("{:?}", token.unwrap().1);
                } */
                let parser = Parser::new();
                let res = parser.parse(lexer);
                match res {
                    Ok(_) => println!("Parsing successful!"),
                    Err(err) => match err {
                        ParseError::InvalidToken { location } => {
                            panic!("Syntax error: invalid token at location {:?}", location);
                        }
                        ParseError::UnrecognizedEof { expected, location } => {
                            panic!(
                                "Syntax error: unrecognized EOF at location {:?}, expected: {}",
                                location,
                                expected.join(", ")
                            );
                        }
                        ParseError::UnrecognizedToken { token, expected } => {
                            let (start, _, end) = token;
                            error(
                                &format!(
                                    "Syntax error: unrecognized token, expected: {{{}}}",
                                    expected.join(", ")
                                ),
                                &get_nth_line(source, start.0),
                                source_path,
                                start,
                                end,
                            );
                        }
                        ParseError::ExtraToken { token } => {
                            let (start, _, end) = token;
                            error(
                                "Syntax error: extra token",
                                &get_nth_line(source, start.0),
                                source_path,
                                start,
                                end,
                            );
                        }
                        ParseError::User { error: inner } => {
                            panic!("Parser error: {}", inner);
                        }
                    },
                }
            }
            Err(err) => fatal(&format!("Memory mapping error: {:?}", err)),
        }
    }
}
