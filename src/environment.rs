use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub(crate) use crate::js_value::JsValue;

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
