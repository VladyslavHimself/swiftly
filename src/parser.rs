use crate::Token;
use crate::ast::ast::{Expression, Program, Statement};

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
            Token::LCURLBRACE => self.parse_block(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            _ => {
                let expr = self.parse_expression();
                self.consume(Token::Semicolon);
                Statement::Expression(expr)
            }
        }
    }


    fn parse_while_statement(&mut self) -> Statement {
        self.consume(Token::While);
        self.consume(Token::LPARENBRACKET);
        let condition = self.parse_expression();
        self.consume(Token::RPARENBRACKET);

        let body = Box::new(self.parse_statement());
        Statement::While { condition, body }
    }

    fn parse_if_statement(&mut self) -> Statement {
        self.consume(Token::If);
        self.consume(Token::LPARENBRACKET);
        let condition = self.parse_expression();
        self.consume(Token::RPARENBRACKET);

        let then_branch = Box::new(self.parse_statement());
        let mut else_branch = None;

        if self.peek() == &Token::Else {
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
        self.consume(Token::LCURLBRACE);
        let mut statements = Vec::new();

        while self.peek() != &Token::RCURLBRACE && self.peek() != &Token::EOF {
            statements.push(self.parse_statement());
        }

        self.consume(Token::RCURLBRACE);
        Statement::Block(statements)
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

    fn parse_primary_expression(&mut self) -> Expression {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;

        match token {
            Token::Number(_) | Token::StringLiteral(_) => Expression::Literal(token.into()),
            Token::Identifier(name) => Expression::Identifier(name),
            _ => panic!("Expected expression"),
        }
    }

    // Prerequisite to A.4 Functions and Classes
    fn parse_expression(&mut self) -> Expression {
        let expr = self.parse_relational_expression();

        if self.peek() == &Token::Assign {
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
        let mut left = self.parse_math_expression();

        while matches!(
            self.peek(),
            Token::RelationOpMore
                | Token::RelationOpMoreOrEqual
                | Token::RelationOpLess
                | Token::RelationOpLessOrEqual
                | Token::EqualityOpEqual
        ) {
            let operator = format!("{:?}", self.peek()); // Тимчасово для простоти
            self.pos += 1;
            let right = self.parse_math_expression();
            left = Expression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
        left
    }

    fn parse_math_expression(&mut self) -> Expression {
        let mut left_expression = self.parse_primary_expression();


        while self.peek() == &Token::Minus {
            self.pos += 1;
            let right = self.parse_primary_expression();

            left_expression = Expression::Binary {
                left: Box::new(left_expression),
                operator: "-".to_string(),
                right: Box::new(right),
            }
        }

        while self.peek() == &Token::Plus {
            self.pos += 1;
            let right = self.parse_primary_expression();

            left_expression = Expression::Binary {
                left: Box::new(left_expression),
                operator: "+".to_string(),
                right: Box::new(right),
            };
        }

        left_expression
    }
}
