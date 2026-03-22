use crate::runtime::Runtime;
use crate::tokens::Token;
use std::fs::File;
use std::io::Read;

pub(crate) mod ast;
mod environment;
mod interpreter;
pub mod js_object;
pub mod js_value;
pub mod lexer;
mod parser;
pub mod runtime;
pub mod tokens;

fn main() {
    println!("[Swiftly]: Running program");
    let mut f = File::open("./examples/example.js").expect("Cannot open file");
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)
        .expect("Something went wrong reading the file");

    Runtime::run(&buffer);
}
