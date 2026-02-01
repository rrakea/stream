mod lexer;

use std::fs;
use std::path;

pub const LANG_NAME: &str = "stream";
pub const FILE_EXTENSION: &str = "str";

fn main() {
    let mut args = std::env::args();

    let _compiler_path = args.next();

    let files_tokens = Vec::new();
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
        let file = fs::File::open(path).expect(format!("File not found: {}", file).as_str());

        let tokens = lexer::lexer::lex(file);
        files_tokens.push(tokens);
    }
}
