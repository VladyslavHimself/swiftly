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

    pub fn assign(&mut self, name: String, value: JsValue) -> bool {
        if self.variables.contains_key(&name) {
            self.variables.insert(name, value);
            return true;
        }

        if let Some(ref parent) = self.parent {
            return parent.borrow_mut().assign(name, value);
        }

        false
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

    pub fn compare(&self, other: &JsValue, op: &str) -> JsValue {
        match (self, other) {
            (JsValue::Number(a), JsValue::Number(b)) => {
                let res = match op {
                    "RelationOpMore" => a > b,
                    "RelationOpMoreOrEqual" => a >= b,
                    "RelationOpLess" => a < b,
                    "RelationOpLessOrEqual" => a <= b,
                    "EqualityOpEqual" => a == b,
                    "EqualityOpNotEqual" => a != b,
                    _ => false,
                };
                JsValue::Boolean(res)
            },

            (JsValue::String(a), JsValue::String(b)) => JsValue::Boolean(a == b),
            _ => JsValue::Boolean(false), // SIMPLIFICATED!
        }
    }

    // !Reference to 7.1.2 ToBoolean
    pub fn is_truthy(&self) -> bool {
        match self {
            JsValue::Boolean(b) => *b,
            JsValue::Number(n) => *n != 0.0, // number add  && !n.is_nan()
            JsValue::String(s) => !s.is_empty(),
            JsValue::Null => false,
            JsValue::Undefined => false,
            _ => false,
        }
    }

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

    pub fn substract(self, other: JsValue) -> JsValue {
        match (self, other) {
            (JsValue::Number(a), JsValue::Number(b)) => JsValue::Number(a - b),
            _ => {
                println!("TypeError: Cannot substract from a non-number value!");
                JsValue::Undefined
            },
        }
    }
}
