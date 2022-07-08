use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::symtbl::{JsFunctionProxy, JsObject, JsValue, Symbol};

pub fn make_object() -> Symbol {
    Symbol::new(
        "Object".to_string(),
        JsValue::Object(Rc::new(RefCell::new(JsObject::new(
            "Object".to_string(),
            HashMap::new(),
            None,
        )))),
    )
}

fn new_object(name: String, proto: Symbol, properties: HashMap<String, JsValue>) -> Symbol {
    Symbol::new(
        name.clone(),
        JsValue::Object(Rc::new(RefCell::new(JsObject::new(
            name,
            properties,
            Some(proto),
        )))),
    )
}

pub struct Console {}

impl Console {
    pub fn make_console(object: Symbol) -> Symbol {
        new_object("console".to_string(), object, Self::make_props())
    }

    fn make_props() -> HashMap<String, JsValue> {
        let mut map = HashMap::new();
        let s = Rc::new(RefCell::new(Self {}));

        {
            let s = s.clone();
            map.insert(
                "log".to_string(),
                JsValue::FunctionProxy(JsFunctionProxy::new(move |params| {
                    s.borrow_mut().log(params)
                })),
            );
        }

        map
    }

    pub fn log(&self, params: &[JsValue]) -> JsValue {
        let value = params.first();
        if let Some(value) = value {
            println!("{}", value.to_string());
        } else {
            println!("")
        }

        JsValue::Undefined
    }
}

pub struct Function;

impl Function {
    pub fn new_js_function()

    pub fn make_function(object: Symbol) -> Symbol {
        new_object("Function".to_string(), object, Self::make_props())
    }
    
    fn make_props() -> HashMap<String, JsValue> {
        let mut map = HashMap::new();
        let s = Rc::new(RefCell::new(Self {}));

        {
            let s = s.clone();
            map.insert(
                "apply".to_string(),
                JsValue::FunctionProxy(JsFunctionProxy::new(move |params| {
                    s.borrow_mut().apply(params)
                })),
            );
        }

        map
    }

    pub fn apply(&self, _: &[JsValue]) -> JsValue {
        println!("apply not implemented");
        JsValue::Undefined
    }
}
