use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use crate::ast::FunctionDeclaration;

#[derive(Clone)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    BigInt,
    Object(Rc<RefCell<JsObject>>),

    // Internal Use
    NativeFunctionProxy(NativeFunctionProxy),
    FunctionDeclaration(FunctionDeclaration),
}

impl JsValue {
    pub fn to_string(&self) -> String {
        match self {
            JsValue::Undefined => format!("undefined"),
            JsValue::Null => format!("null"),
            JsValue::Boolean(b) => format!("{}", b),
            JsValue::String(s) => format!("{}", s),
            JsValue::Number(n) => format!("{}", n),
            JsValue::BigInt => format!("BitInt"),
            JsValue::Object(_) => format!("[Object object]"),
            JsValue::NativeFunctionProxy(_) => format!("[native proxy]"),
            JsValue::FunctionDeclaration(f) => {
                format!("function {}", f.name.as_deref().unwrap_or("[Anonymous]"))
            }
        }
    }
}

#[derive(Clone)]
pub struct Symbol {
    name: String,
    value: JsValue,
}

impl Symbol {
    pub fn new(name: String, value: JsValue) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> JsValue {
        self.value.clone()
    }

    pub fn get_property_value(&self, property: &str) -> JsValue {
        match &self.value {
            JsValue::Object(object) => object.borrow_mut().get_property_value(property),
            _ => JsValue::Undefined,
        }
    }
}

pub struct JsObject {
    name: String,
    properties: HashMap<String, JsValue>,
    prototype: Option<Symbol>,
    is_function: bool,
}

impl JsObject {
    pub fn new(
        name: String,
        properties: HashMap<String, JsValue>,
        prototype: Option<Symbol>,
    ) -> Self {
        Self {
            name,
            properties,
            prototype,
            is_function: false,
        }
    }

    pub fn get_property_value(&self, property: &str) -> JsValue {
        self.properties
            .get(property)
            .and_then(|s| Some(s.clone()))
            .unwrap_or(JsValue::Undefined)
    }

    pub fn is_function(&self) -> bool {
        self.is_function
    }

    pub fn set_is_function(&mut self, is_function: bool) {
        self.is_function = is_function
    }
}

#[derive(Clone)]
pub struct NativeFunctionProxy {
    function: Rc<RefCell<dyn FnMut(&[JsValue]) -> JsValue>>,
}

impl NativeFunctionProxy {
    pub fn new<F: 'static + FnMut(&[JsValue]) -> JsValue>(function: F) -> Self {
        Self {
            function: Rc::new(RefCell::new(function)),
        }
    }

    pub fn function(&mut self) -> RefMut<dyn FnMut(&[JsValue]) -> JsValue> {
        self.function.borrow_mut()
    }
}

/*#[derive(Clone)]
pub struct SymbolHandle {
    path: Vec<String>,
}

impl SymbolHandle {
    pub fn new(path: Vec<String>) -> Self {
        Self { path }
    }

    pub fn current(&self) -> Option<&str> {
        self.path.get(0).map(|s| s.as_str())
    }

    pub fn pop(self) -> Self {
        Self {
            path: self.path.into_iter().skip(1).collect(),
        }
    }
}

pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /*pub fn get_symbol(&self, handle: SymbolHandle) -> Option<&Symbol> {
        Self::_get_symbol(self, handle)
    }

    fn _get_symbol(symbol: &Symbol, handle: SymbolHandle) -> Option<&Symbol> {
        let next = handle.clone().pop();
        handle
            .current()
            .and_then(|h| match &symbol.value {
                JsValue::Object(object) => object
                    .properties
                    .symbols
                    .get(h)
                    .and_then(|s| Self::_get_symbol(s, next)),
                _ => None,
            })
            .or(Some(symbol))
    }*/
}*/
