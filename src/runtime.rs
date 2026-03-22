use crate::ast::ast::Program;
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::tokens::Token;

pub struct Runtime;

impl Runtime {
    pub fn run(source: &str) {
        let mut lexer = Lexer::new(source);
        let tokens = Self::lex(&mut lexer);
        let program = Self::parse(tokens);
        Self::execute(program);
    }

    fn lex(lexer: &mut Lexer) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            println!("{:?}", token);
            let is_eof = token == Token::EOF;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn parse(tokens: Vec<Token>) -> Program {
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    fn execute(program: Program) {
        let mut interpreter = Interpreter::new();
        println!("[Swiftly-Info]: AST: {:#?}", program.body);
        interpreter.execute(program.body);
    }
}
