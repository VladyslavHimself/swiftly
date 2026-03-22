use crate::Token;
use crate::ast::ast::{Expression, Program, Statement};
use crate::environment::JsValue;
use crate::tokens::Keyword::{Else, If, Let, While};
use crate::tokens::Literal::{JNumber, JString};
use crate::tokens::Operator::{
    Assign, Equal, Greater, GreaterOrEqual, Less, LessOrEqual, Minus, Not, Percent, Plus, Slash,
    Star,
};
use crate::tokens::Punctuation::{Colon, Comma, Dot, LBrace, LParen, RBrace, RParen, Semicolon};

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

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        if token != Token::EOF {
            self.pos += 1;
        }
        token
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
            Token::Keyword(Let) => self.parse_variable_declaration(),
            Token::Punctuation(LBrace) => self.parse_block(),
            Token::Keyword(If) => self.parse_if_statement(),
            Token::Keyword(While) => self.parse_while_statement(),
            _ => {
                let expr = self.parse_expression();
                self.consume(Token::Punctuation(Semicolon));
                Statement::Expression(expr)
            }
        }
    }

    fn parse_while_statement(&mut self) -> Statement {
        self.consume(Token::Keyword(While));
        self.consume(Token::Punctuation(LParen));
        let condition = self.parse_expression();
        self.consume(Token::Punctuation(RParen));

        let body = Box::new(self.parse_statement());
        Statement::While { condition, body }
    }

    fn parse_if_statement(&mut self) -> Statement {
        self.consume(Token::Keyword(If));
        self.consume(Token::Punctuation(LParen));
        let condition = self.parse_expression();
        self.consume(Token::Punctuation(RParen));

        let then_branch = Box::new(self.parse_statement());
        let mut else_branch = None;

        if self.peek() == &Token::Keyword(Else) {
            self.pos += 1;
            else_branch = Some(Box::new(self.parse_statement()));
        }

        Statement::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn parse_block(&mut self) -> Statement {
        self.consume(Token::Punctuation(LBrace));
        let mut statements = Vec::new();

        while self.peek() != &Token::Punctuation(RBrace) && self.peek() != &Token::EOF {
            statements.push(self.parse_statement());
        }

        self.consume(Token::Punctuation(RBrace));
        Statement::Block(statements)
    }

    fn parse_variable_declaration(&mut self) -> Statement {
        self.consume(Token::Keyword(Let));

        let id = if let Token::Identifier(name) = self.peek().clone() {
            self.pos += 1;
            // &String to String (Temporary)
            name.clone()
        } else {
            panic!("Expected identifier after let");
        };

        self.consume(Token::Operator(Assign));

        let init = self.parse_expression();

        self.consume(Token::Punctuation(Semicolon));

        Statement::VariableDeclaration { id, init }
    }

    fn parse_primary_expression(&mut self) -> Expression {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;

        match token {
            Token::Literal(JNumber(_)) | Token::Literal(JString(_)) => {
                Expression::Literal(token.into())
            }
            Token::Identifier(name) => Expression::Identifier(name),
            Token::Punctuation(LParen) => {
                let expr = self.parse_expression();

                if self.peek() == &Token::Punctuation(RParen) {
                    self.pos += 1;
                    expr
                } else {
                    panic!("Expected ')' after expression");
                }
            }
            Token::Punctuation(LBrace) => {
                let mut properties = Vec::new();

                while self.peek() != &Token::Punctuation(RBrace) {
                    let key = match self.advance() {
                        Token::Identifier(name) => name,
                        _ => panic!("Expected identifier as object key"),
                    };

                    self.consume(Token::Punctuation(Colon));

                    let value = self.parse_expression();
                    properties.push((key, value));
                    if self.peek() == &Token::Punctuation(Comma) {
                        self.pos += 1;
                    }
                }

                self.pos += 1;
                Expression::ObjectLiteral(properties)
            }
            _ => panic!("Expected expression, but got {:?}", self.peek()),
        }
    }

    fn parse_member_expression(&mut self) -> Expression {
        let mut expr = self.parse_primary_expression();

        while self.peek() == &Token::Punctuation(Dot) {
            self.pos += 1;

            let property = match self.advance() {
                Token::Identifier(name) => name,
                _ => panic!("Expected identifier after '.'"),
            };

            expr = Expression::MemberAccess {
                object: Box::new(expr),
                property,
            };
        }

        expr
    }

    // Prerequisite to A.4 Functions and Classes
    fn parse_expression(&mut self) -> Expression {
        let expr = self.parse_relational_expression();

        if self.peek() == &Token::Operator(Assign) {
            self.pos += 1;

            let value = self.parse_expression();
            if let Expression::Identifier(name) = expr {
                return Expression::Assignment {
                    name,
                    value: Box::new(value),
                };
            }

            panic!("Invalid assignment target!");
        }

        expr
    }

    fn parse_relational_expression(&mut self) -> Expression {
        let mut left = self.parse_additive_expression();

        while matches!(
            self.peek(),
            Token::Operator(Greater)
                | Token::Operator(GreaterOrEqual)
                | Token::Operator(Less)
                | Token::Operator(LessOrEqual)
                | Token::Operator(Equal)
        ) {
            let operator = format!("{:?}", self.peek()); // Тимчасово для простоти
            self.pos += 1;
            let right = self.parse_additive_expression();
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_additive_expression(&mut self) -> Expression {
        let mut left_expression = self.parse_multiplicative_expression();

        while self.peek() == &Token::Operator(Plus) || self.peek() == &Token::Operator(Minus) {
            let operator = if self.peek() == &Token::Operator(Plus) {
                "+"
            } else {
                "-"
            }
            .to_string();
            self.pos += 1;

            let right = self.parse_multiplicative_expression();

            left_expression = Expression::Binary {
                left: Box::new(left_expression),
                operator,
                right: Box::new(right),
            };
        }

        left_expression
    }

    fn parse_multiplicative_expression(&mut self) -> Expression {
        let mut left = self.parse_unary_expression();

        while matches!(
            self.peek(),
            Token::Operator(Star) | Token::Operator(Slash) | Token::Operator(Percent)
        ) {
            let op = format!("{:?}", self.peek());
            self.pos += 1;
            let right = self.parse_unary_expression();
            left = Expression::Binary {
                left: Box::new(left),
                operator: op,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_unary_expression(&mut self) -> Expression {
        if matches!(self.peek(), Token::Operator(Minus) | Token::Operator(Not)) {
            let op = format!("{:?}", self.peek());
            self.pos += 1;
            // TODO: Рекурсивно кличемо parse_unary_expression, щоб обробити --5 UPD: (має бути помилка)
            // TODO: Should be: Uncaught SyntaxError: Invalid left-hand side expression in prefix operation
            let right = self.parse_unary_expression();
            return Expression::Unary {
                operator: op,
                operand: Box::new(right),
            };
        }

        self.parse_member_expression()
    }
}
