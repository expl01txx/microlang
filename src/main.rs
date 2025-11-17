mod interpreter;
mod lexer;
mod parser;
mod preprocessor;

use clap::{command, Parser};
use lexer::Token;
use logos::Logos;

use crate::{interpreter::Interpreter, parser::Parser as MicroParser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(long, default_value_t = false)]
    stacktrace: bool,

    #[arg(long, default_value_t = false)]
    print_instructions: bool,
    
    #[arg(long, default_value_t = false)]
    print_preprocessed: bool,
}

fn main() {
    let args = Args::parse();

    let source = std::fs::read_to_string(args.filename).unwrap();

    let preprocessed = preprocessor::preprocess(source);

    let lex = Token::lexer(&preprocessed);

    let mut parser = MicroParser::new(lex);
    let insts = parser.parse();

    let mut interpreter = Interpreter::new(insts);
    interpreter.execute(args.stacktrace);
}
