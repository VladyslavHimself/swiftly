use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fs::File;
use std::io::Read;
use crate::tokens::Token;

mod environment;
mod interpreter;
pub mod lexer;
mod parser;
pub mod tokens;
pub(crate) mod ast;
pub mod js_value;

fn main() {
    println!("[Swiftly-Info]: Reading file...");
    let mut f = File::open("./examples/example.js").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    println!("[Swiftly-Info]: File read successfully!");

    // Lexer
    let mut lexer: Lexer = Lexer::new(&buffer);
    let mut tokens = Vec::new();

    println!("[Swiftly-Info]: Generating tokens...");
    loop {
        let token: Token = lexer.next_token();
        println!("{:?}", token);
        let is_eof = token == Token::EOF;
        tokens.push(token);
        if is_eof {
            break;
        }
    }
    println!("[Swiftly-Info]: Tokens generated successfully!");
    // ---


    // Parser
    println!("[Swiftly-Info]: Parsing program...");
    let mut parser = Parser::new(tokens);
    let program = parser.parse_program();
    println!("[Swiftly-Info]: AST: {:#?}", program.body);

    // ---

    println!("[Swiftly-Info]: Evaluating...");
    let mut interpreter = Interpreter::new();
    interpreter.execute(program.body);
    println!("[Swiftly-Info]: Done!");
}
