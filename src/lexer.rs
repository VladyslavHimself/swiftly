use crate::Token;
use crate::tokens::Keyword::{Else, If, Let, While};
use crate::tokens::Literal::{JBoolean, JNumber, JString};
use crate::tokens::Operator::{
    Assign, Equal, Greater, GreaterOrEqual, Less, LessOrEqual, Minus, Not, NotEqual, Percent, Plus,
    Slash, Star,
};
use crate::tokens::Punctuation::{LBrace, LParen, RBrace, RParen, Semicolon};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub(crate) fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.pos];

        // Keywords & words detection
        if ch.is_alphabetic() {
            let word = self.read_identifier();
            return match word.as_str() {
                "let" => Token::Keyword(Let),
                "true" => Token::Literal(JBoolean(true)),
                "false" => Token::Literal(JBoolean(false)),
                "if" => Token::Keyword(If),
                "else" => Token::Keyword(Else),
                "while" => Token::Keyword(While),

                _ => Token::Identifier(word),
            };
        }

        if ch.is_numeric() {
            return Token::Literal(JNumber(self.read_number()));
        }

        if ch == '"' {
            self.pos += 1;
            let value = self.read_string_value();
            self.pos += 1;
            return Token::Literal(JString(value));
        }

        match ch {
            '=' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::Operator(Equal)
                } else {
                    self.pos += 1;
                    Token::Operator(Assign)
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::Operator(NotEqual)
                } else {
                    self.pos += 1;
                    Token::Operator(Not)
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::Operator(GreaterOrEqual)
                } else {
                    self.pos += 1;
                    Token::Operator(Greater)
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::Operator(LessOrEqual)
                } else {
                    self.pos += 1;
                    Token::Operator(Less)
                }
            }

            '+' => {
                self.pos += 1;
                Token::Operator(Plus)
            }

            '-' => {
                self.pos += 1;
                Token::Operator(Minus)
            }

            '*' => {
                self.pos += 1;
                Token::Operator(Star)
            }

            '/' => {
                // TODO: Add support for comments
                self.pos += 1;
                Token::Operator(Slash)
            }

            '%' => {
                self.pos += 1;
                Token::Operator(Percent)
            }

            ';' => {
                self.pos += 1;
                Token::Punctuation(Semicolon)
            }

            '(' => {
                self.pos += 1;
                Token::Punctuation(LParen)
            }

            ')' => {
                self.pos += 1;
                Token::Punctuation(RParen)
            }

            '{' => {
                self.pos += 1;
                Token::Punctuation(LBrace)
            }
            '}' => {
                self.pos += 1;
                Token::Punctuation(RBrace)
            }

            _ => panic!("Unknown character: {}", ch),
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;

        while self.pos < self.input.len() && self.input[self.pos].is_alphanumeric() {
            self.pos += 1;
        }
        self.input[start..self.pos].iter().collect()
    }

    fn read_string_value(&mut self) -> String {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            self.pos += 1;
        }

        self.input[start..self.pos].iter().collect()
    }

    fn read_number(&mut self) -> f64 {
        let start = self.pos;
        while self.pos < self.input.len() && self.input[self.pos].is_numeric()
            || self.input[self.pos] == '.'
        {
            self.pos += 1;
        }
        let s: String = self.input[start..self.pos].iter().collect();
        s.parse().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        if self.pos + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.pos + 1])
        }
    }
}
