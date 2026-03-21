use crate::Token;

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
                "let" => Token::Let,
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                "if" => Token::If,
                "else" => Token::Else,
                "while" => Token::While,

                _ => Token::Identifier(word),
            };
        }

        if ch.is_numeric() {
            return Token::Number(self.read_number());
        }

        if ch == '"' {
            self.pos += 1;
            let value = self.read_string_value();
            self.pos += 1;
            return Token::StringLiteral(value);
        }

        match ch {
            '=' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::EqualityOpEqual
                } else {
                    self.pos += 1;
                    Token::Assign
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::EqualityOpNotEqual
                } else {
                    self.pos += 1;
                    Token::Not
                }
            }

            '>' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::RelationOpMoreOrEqual
                } else {
                    self.pos += 1;
                    Token::RelationOpMore
                }
            }

            '<' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::RelationOpLessOrEqual
                } else {
                    self.pos += 1;
                    Token::RelationOpLess
                }
            }

            '+' => {
                self.pos += 1;
                Token::Plus
            }

            '-' => {
                self.pos += 1;
                Token::Minus
            }

            ';' => {
                self.pos += 1;
                Token::Semicolon
            }

            '(' => {
                self.pos += 1;
                Token::LPARENBRACKET
            }

            ')' => {
                self.pos += 1;
                Token::RPARENBRACKET
            }

            '{' => {
                self.pos += 1;
                Token::LCURLBRACE
            }
            '}' => {
                self.pos += 1;
                Token::RCURLBRACE
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
