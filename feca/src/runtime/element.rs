use std::collections::HashMap;

use catus::{
    builtins::{new_object, Function},
    symtbl::{JsValue, NativeFunctionProxy, Symbol},
};
use crosscom::ComRc;
use felis::{comdef::IElement, DomString};

pub fn make_element(object: Symbol, function: Symbol, element: ComRc<IElement>) -> Symbol {
    let mut props = HashMap::new();

    {
        let element = element.clone();
        props.insert(
            "setInnerHTML".to_string(),
            Function::new_native_proxy_function(
                function,
                "setInnerHTML".to_string(),
                NativeFunctionProxy::new(move |params| set_inner_html(element.clone(), params)),
            )
            .value(),
        );
    }

    new_object("document".to_string(), object, props)
}

fn set_inner_html(element: ComRc<IElement>, params: &[JsValue]) -> JsValue {
    println!("in set inner html");
    let text = params.get(0);

    if let Some(JsValue::String(text)) = text {
        element.set_inner_html(DomString::new(text.to_string()));
    }

    JsValue::Undefined
}
