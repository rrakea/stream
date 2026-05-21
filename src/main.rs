mod ast;
mod lexer;
mod llvm;
mod optimizer;
mod parser;
mod typechecker;

use std::fs;
use std::path;

use crate::lexer::lexer::lex;
use crate::parser::parser::parse;

pub const LANG_NAME: &str = "stream";
pub const FILE_EXTENSION: &str = "str";

fn main() {
    let mut args = std::env::args();

    let _compiler_path = args.next();

    let mut files_tokens = Vec::new();
    for path_str in args {
        let path = path::Path::new(&path_str);
        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap();
            if ext != FILE_EXTENSION {
                println!(
                    "File extension incorrect: {} Expected: {}",
                    ext, FILE_EXTENSION
                );
                return;
            }
        } else {
            println!("File without extension: {}", path_str);
        }
        let file = fs::File::open(path).expect("File not found");

        let tokens = match lex(file) {
            Ok(t) => t,
            Err(err) => {
                println!("Lexer error in file: {}. Error: {}", path_str, err);
                return;
            }
        };
        files_tokens.push(tokens);
    }

    for tokens in files_tokens {
        let parse_res = parse(tokens).unwrap();
    }
}
