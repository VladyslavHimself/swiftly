mod ast;

use crate::Token;
use crate::parser::ast::ast::{Expression, Program, Statement};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    // Helper method to peek current token
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self, expected: Token) {
        if self.tokens[self.pos] == expected {
            self.pos += 1;
        } else {
            panic!(
                "Expected {:?}, but got {:?}",
                expected, self.tokens[self.pos]
            );
        }
    }

    pub(crate) fn parse_program(&mut self) -> Program {
        let mut body = Vec::new();
        while self.peek() != &Token::EOF {
            body.push(self.parse_statement());
        }

        Program { body }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.peek() {
            Token::Let => self.parse_variable_declaration(),
            _ => panic!("Unexpected token {:?}", self.peek()),
        }
    }

    fn parse_variable_declaration(&mut self) -> Statement {
        self.consume(Token::Let);

        let id = if let Token::Identifier(name) = self.peek().clone() {
            self.pos += 1;
            // &String to String (Temporary)
            name.clone()
        } else {
            panic!("Expected identifier after let");
        };

        self.consume(Token::Assign);

        let init = self.parse_expression();

        self.consume(Token::Semicolon);

        Statement::VariableDeclaration { id, init }
    }

    fn parse_expression(&mut self) -> Expression {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;

        match token {
            Token::Number(_) | Token::StringLiteral(_) => Expression::Literal(token),
            Token::Identifier(name) => Expression::Identifier(name),
            _ => panic!("Expected expression"),
        }

    }
}
