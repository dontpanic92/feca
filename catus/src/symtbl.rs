use std::collections::HashMap;


pub enum JsType {
    Undefined,
    Null,
    Boolean(bool),
    String(String),
    Number(f64),
    BigInt,
    Object(JsObject),
}

pub struct Symbol {
    name: String,
    value: JsType,
}

pub struct JsObject {
    name: String,
    properties: SymbolTable,
    prototype: SymbolHandle,
}

#[derive(Clone)]
pub struct SymbolHandle {
    path: Vec<String>,
}

pub type SymbolTable = HashMap<String, Symbol>;
