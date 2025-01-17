mod environment;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        panic!("[ERROR] too much arguments");
    }

    let filename = &args[1];

    let lexer = Lexer::for_file(filename);
    let mut parser = Parser::new(lexer);

    for s in parser {
        println!("{}", s);
    }
}
