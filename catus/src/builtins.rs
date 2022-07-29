use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    ast::FunctionDeclaration,
    symtbl::{JsObject, JsValue, NativeFunctionProxy, Symbol},
};

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

pub fn new_object(name: String, proto: Symbol, properties: HashMap<String, JsValue>) -> Symbol {
    let mut obj = JsObject::new(name.clone(), properties, Some(proto));
    obj.set_is_function(true);

    Symbol::new(name, JsValue::Object(Rc::new(RefCell::new(obj))))
}

pub struct Console {}

impl Console {
    pub fn make_console(object: Symbol, function: Symbol) -> Symbol {
        new_object("console".to_string(), object, Self::make_props(function))
    }

    fn make_props(function: Symbol) -> HashMap<String, JsValue> {
        let mut map = HashMap::new();
        let s = Rc::new(RefCell::new(Self {}));

        {
            let s = s.clone();
            map.insert(
                "log".to_string(),
                Function::new_native_proxy_function(
                    function,
                    "log".to_string(),
                    NativeFunctionProxy::new(move |params| s.borrow_mut().log(params)),
                )
                .value(),
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
    pub fn new_js_function(
        object: Symbol,
        function_proto: Symbol,
        decl: &FunctionDeclaration,
    ) -> Symbol {
        let mut properties = HashMap::new();
        let prototype = new_object("prototype".to_string(), object, HashMap::new());

        properties.insert(
            "__function__".to_string(),
            JsValue::FunctionDeclaration(decl.clone()),
        );

        properties.insert("prototype".to_string(), prototype.value());

        new_object(
            decl.name.as_ref().unwrap().clone(),
            function_proto.clone(),
            properties,
        )
    }

    pub fn new_native_proxy_function(
        function_proto: Symbol,
        name: String,
        proxy: NativeFunctionProxy,
    ) -> Symbol {
        let mut properties = HashMap::new();

        properties.insert(
            "__function__".to_string(),
            JsValue::NativeFunctionProxy(proxy),
        );

        new_object(name, function_proto.clone(), properties)
    }

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
                JsValue::NativeFunctionProxy(NativeFunctionProxy::new(move |params| {
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
