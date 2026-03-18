use crate::Token;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, f64>,
    strings: HashMap<String, String>,
}

impl Environment {
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
            strings: HashMap::new(),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: Token) {
        match value {
            Token::Number(n) => {
                self.variables.insert(name, n);
            }
            Token::StringLiteral(s) => {
                self.strings.insert(name, s);
            }
            _ => panic!("Invalid variable type"),
        }
    }
}
