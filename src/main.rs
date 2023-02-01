use std::process::exit;

use codegen::Codegen;

use crate::{
    display::display_error,
    parser::{ParseError, Parser},
};

mod display;
mod node;
mod parser;
mod codegen;
mod token;
mod tokenizer;

fn main() {
    let args: Vec<String> = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        panic!("Wrong number of arguments.");
    }

    let tokens = match crate::tokenizer::tokenize(&args[1]) {
        Ok(tokens) => tokens,
        Err(error) => {
            display_error(error.source, error.position, &error.message);
            exit(1)
        }
    };
    // println!("{:?}", &tokens);

    let mut parser = Parser::new(tokens.clone());
    let statements = match parser.parse_program() {
        Ok(statements) => statements,
        Err(ParseError(token, message)) => {
            display_error(&args[1], tokens[token].position, &message);
            exit(1)
        }
    };
    // println!("{:?}", statements);
    
    let code = Codegen::gen(statements);
    println!("{}", code);
}
