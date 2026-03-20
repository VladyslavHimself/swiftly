use crate::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum JsValue {
    Number(f64),
    String(String),
    Undefined,
    Null,
    Boolean(bool),
    // TODO: Add objects
}

#[derive(Debug, Clone)]
pub struct Environment {
    variables: HashMap<String, JsValue>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    // Should refer to #6 ECMAScript Data Types and Values (JSValue)
    pub(crate) fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_enclosed(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub(crate) fn define(&mut self, name: String, value: JsValue) {
        self.variables.insert(name, value);
    }

    pub(crate) fn get(&self, name: &str) -> JsValue {
        if let Some(value) = self.variables.get(name) {
            return value.clone();
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow().get(name);
        }

        // TODO: Should be ReferenceError
        println!("ReferenceError: {} is not defined", name);
        JsValue::Undefined
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
