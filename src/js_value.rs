use crate::js_object::JsObject;
use crate::tokens::Literal::{JNumber, JString};
use crate::tokens::Token;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum JsValue {
    Number(f64),
    String(String),
    Undefined,
    Null,
    Boolean(bool),

    // 6.1.7: The Object Type
    Object(Rc<RefCell<JsObject>>),
}

impl PartialEq for JsValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JsValue::Number(a), JsValue::Number(b)) => a == b,
            (JsValue::String(a), JsValue::String(b)) => a == b,
            (JsValue::Boolean(a), JsValue::Boolean(b)) => a == b,
            (JsValue::Null, JsValue::Null) => true,
            (JsValue::Undefined, JsValue::Undefined) => true,
            // Reference Equality
            (JsValue::Object(a), JsValue::Object(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}


impl From<Token> for JsValue {
    fn from(token: Token) -> Self {
        match token {
            Token::Literal(JNumber(n)) => JsValue::Number(n),
            Token::Literal(JString(s)) => JsValue::String(s),
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
                    "Greater" => a > b,
                    "GreaterOrEqual" => a >= b,
                    "Less" => a < b,
                    "LessOrEqual" => a <= b,
                    "Equal" => a == b,
                    "NotEqual" => a != b,
                    _ => false,
                };
                JsValue::Boolean(res)
            }

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
            JsValue::Null | JsValue::Undefined => false,
            JsValue::Object(_) => true,
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
            }
        }
    }

    pub fn binary_op(self, other: JsValue, op: &str) -> JsValue {
        match (self, other) {
            (JsValue::Number(a), JsValue::Number(b)) => match op {
                "Plus" => JsValue::Number(a + b),
                "Minus" => JsValue::Number(a - b),
                "Star" => JsValue::Number(a * b),
                "Slash" => {
                    if b == 0.0 {
                        JsValue::Number(f64::INFINITY)
                    } else {
                        JsValue::Number(a / b)
                    }
                }
                "Percent" => JsValue::Number(a % b),
                _ => JsValue::Undefined,
            },
            _ => JsValue::Undefined,
        }
    }
}
