#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),        // For numbers (For JS everything is f64) :- 6.1.6.1 The Number Type
    StringLiteral(String), // For string literals :- 12.9.4 String Literals
    Identifier(String), // identifiers names for vars and functions :- BindingIdentifier
    Let,                // "let" keyword
    Assign,             // Operator =
    Semicolon,          // semicolon - ;

    Equal,    // ==
    Not,      // !
    NotEqual, // !=
    EOF,      // End of file
}

#[derive(Debug)]
struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Token {
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
                    Token::Equal
                } else {
                    self.pos += 1;
                    Token::Assign
                }
            }

            '!' => {
                if self.peek() == Some('=') {
                    self.pos += 2;
                    Token::NotEqual
                } else {
                    self.pos += 1;
                    Token::Not
                }
            }

            ';' => {
                self.pos += 1;
                Token::Semicolon
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

fn main() {
    // TODO: Get code from file;
    let code = "let s = \"hello world\";";
    let mut lexer: Lexer = Lexer::new(code);

    print!("{:?}", lexer);

    loop {
        let token: Token = lexer.next_token();
        println!("{:?}", token);

        if token == Token::EOF {
            break;
        }
    }
}
