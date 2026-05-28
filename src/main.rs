use clap::Parser;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::BufRead;
use logos::Logos;


#[derive(Parser)]
struct Args {
    /// The pattern to look for
    #[arg(short, long)]
    lex: bool,
    /// The path to the file to read
    path: std::path::PathBuf,
}
#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[regex("[a-zA-Z_]+")]
    Identifier,
    #[regex("[0-9]+")]
    Constant,
    #[token("int")]
    Int,
    #[token("return")]
    Return,
    #[token("void")]
    Void,
    #[token(";")]
    Semicolon,
    #[token("(")]
    OpenParenthesis,
    #[token(")")]
    CloseParenthesis,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
}

fn main() {

    let args = Args::parse();


    if args.lex {
        println!("Using lexer...");
        let path = Path::new(&args.path);
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let is_c_file = path.extension().map_or(false, |ext| ext == "c");
        if is_c_file {
            let file = File::open(&path).expect("Could not open the file");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.expect("Could not read the line");
                for token in Token::lexer(&line) {
                    match token {
                        Ok(tok) => println!("{:?}", tok),
                        Err(err) => println!("Error: {:?}", err),

                    }
                }
            }
        }
        else {
            println!("Unsupported file type: {}", ext);
        }
    }
}
