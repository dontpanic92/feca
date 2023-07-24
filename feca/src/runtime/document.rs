use std::collections::HashMap;

use catus::{
    builtins::{new_object, Function},
    symtbl::{JsValue, NativeFunctionProxy, Symbol},
};
use crosscom::ComRc;
use felis::{comdef::INode, DomString};

use super::element::make_element;

pub fn make_document(object: Symbol, function: Symbol, document: ComRc<INode>) -> Symbol {
    let mut props = HashMap::new();

    {
        let object = object.clone();
        let function = function.clone();
        let document = document.clone();
        props.insert(
            "getElementById".to_string(),
            Function::new_native_proxy_function(
                function.clone(),
                "getElementById".to_string(),
                NativeFunctionProxy::new(move |params| {
                    get_element_by_id(document.clone(), object.clone(), function.clone(), params)
                }),
            )
            .value(),
        );
    }

    new_object("document".to_string(), object.clone(), props)
}

fn get_element_by_id(
    document: ComRc<INode>,
    object: Symbol,
    function: Symbol,
    params: &[JsValue],
) -> JsValue {
    let id = params.get(0);
    if id.is_none() {
        return JsValue::Undefined;
    }

    let element = document.get_element_by_id(DomString::new(id.unwrap().to_string()));
    if let Some(element) = element {
        make_element(object, function, element).value()
    } else {
        JsValue::Undefined
    }
}
