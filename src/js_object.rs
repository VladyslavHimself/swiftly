use crate::js_value::JsValue;
use std::collections::HashMap;

#[derive(Debug)]
pub struct JsObject {
    pub prototype: JsValue, // [[Prototype]] - can be Object or null
    pub extensible: bool,   // [[IsExtensible]]

    // TODO: Add another Essential Internal Methods later
    // TODO: !Reference to 6.1.7.2 Object Internal Methods and Internal Slots
    pub properties: HashMap<String, PropertyDescriptor>,
}


// !Reference to 6.1.7.1 Property Attributes
#[derive(Debug, Clone)]
pub struct PropertyDescriptor {
    pub value: Option<JsValue>,
    pub writable: Option<bool>,
    pub get: Option<JsValue>, // Must be Callable
    pub set: Option<JsValue>, // Also must be Callable
    pub enumerable: Option<bool>,
    pub configurable: Option<bool>,

    // TODO: Add getters/setters for Accessor Properties
}

impl JsObject {
    pub fn new(prototype: JsValue) -> Self {
        Self {
            prototype,
            extensible: true,
            properties: HashMap::new(),
        }
    }

    // !Reference to 10.1.8 | [[Get]] ( P, Receiver )
    pub fn get_property(&self, key: &str) -> JsValue {
        if let Some(desc) = self.properties.get(key) {

            // find own property
            if let Some(value) = &desc.value {
                return value.clone();
            }
        }

        // if didn't find, then go to prototype
        match &self.prototype {
           JsValue::Object(proto) => proto.borrow().get_property(key),
            _ => JsValue::Undefined // TODO: Check what should be returned here (Null or Undefined)
        }
    }

    // !Reference to 10.1.9 | [[Set]] ( P, V, Receiver )
    // TODO: Simplified version without Accessors
    pub fn set_property(&mut self, key: String, value: JsValue) {

        // TODO: Now check just for own propery. Add prototype chaining lookup
        if let Some(desc) = self.properties.get_mut(&key) {
            // TODO: Check for unwrap
            if desc.writable == Some(true) {
                desc.value = Some(value);
            }
            return;
        }

        if self.extensible {
            self.properties.insert(key, PropertyDescriptor {
                value: Some(value),
                writable: Some(true),
                get: None,
                set: None,
                enumerable: Some(true),
                configurable: Some(true),
            });
        }
    }
}