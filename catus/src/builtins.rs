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

pub struct Console {}

impl Console {
    pub fn make_console(object: Symbol) -> Symbol {
        Symbol::new(
            "console".to_string(),
            JsValue::Object(Rc::new(RefCell::new(JsObject::new(
                "console".to_string(),
                Self::make_props(),
                Some(object),
            )))),
        )
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
