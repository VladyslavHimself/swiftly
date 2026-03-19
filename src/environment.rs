use crate::Token;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum JsValue {
    Number(f64),
    String(String),
    Undefined,
    Null,
    Boolean(bool),
    // TODO: Add objects
}

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<String, JsValue>,
}

impl Environment {
    // Should refer to #6 ECMAScript Data Types and Values (JSValue)
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: JsValue) {
        self.variables.insert(name, value);
    }

    pub(crate) fn get(&self, name: &str) -> JsValue {
        self.variables.get(name).cloned().unwrap_or(JsValue::Undefined)
    }
}

impl From<Token> for JsValue {
   fn from(token: Token) -> Self {
       match token {
           Token::Number(n) => JsValue::Number(n),
           Token::StringLiteral(s) => JsValue::String(s),
           // TODO: Add more types
           _ => JsValue::Undefined,
       }
   }
}

impl JsValue {

    // TODO: Add more operators
    // !Reference to 13.15.3 ApplyStringOrNumericBinaryOperator

    pub fn add(self, other: JsValue) -> JsValue {
        match (self, other) {
            (JsValue::Number(a), JsValue::Number(b)) => JsValue::Number(a + b),
            (JsValue::String(a), JsValue::String(b)) => JsValue::String(format!("{}{}", a, b)),
            (JsValue::String(a), JsValue::Number(b)) => JsValue::String(format!("{}{}", a, b)),
            (JsValue::Number(a), JsValue::String(b)) => JsValue::String(format!("{}{}", a, b)),
            _ => JsValue::Undefined,
        }
    }
}