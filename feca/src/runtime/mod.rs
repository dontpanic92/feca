use std::time::{Duration, Instant};

use catus::{
    builtins::Function,
    interpreter::Interpreter,
    symtbl::{JsValue, NativeFunctionProxy, Symbol},
};
use crosscom::ComRc;
use felis::comdef::INode;

use self::{date::make_date, document::make_document, timer_queue::TIMER_QUEUE};

pub mod date;
pub mod document;
pub mod element;
pub mod timer_queue;

pub fn setup_js_runtime(interpreter: &mut Interpreter, document: ComRc<INode>) {
    TIMER_QUEUE.with(|q| q.borrow_mut().clear());

    let globals = interpreter.global_symbols();
    let object = globals.get("Object").unwrap().clone();
    let function = globals.get("Function").unwrap().clone();

    globals.insert("setTimeout".to_string(), make_set_timeout(function.clone()));
    globals.insert(
        "document".to_string(),
        make_document(object.clone(), function.clone(), document.clone()),
    );
    globals.insert(
        "Date".to_string(),
        make_date(object.clone(), function.clone()),
    );
}

fn make_set_timeout(function: Symbol) -> Symbol {
    Function::new_native_proxy_function(
        function,
        "setTimeout".to_string(),
        NativeFunctionProxy::new(set_timeout),
    )
}

fn set_timeout(params: &[JsValue]) -> JsValue {
    let function = params.get(0);
    let delay = params
        .get(1)
        .map(|v| match v {
            JsValue::Undefined => todo!(),
            JsValue::Null => todo!(),
            JsValue::Boolean(_) => todo!(),
            JsValue::String(_) => todo!(),
            JsValue::Number(n) => *n,
            JsValue::BigInt => todo!(),
            JsValue::Object(_) => todo!(),
            JsValue::NativeFunctionProxy(_) => todo!(),
            JsValue::FunctionDeclaration(_) => todo!(),
        })
        .unwrap_or(0.);

    match function {
        Some(JsValue::Object(obj)) => TIMER_QUEUE.with(|q| {
            let mut queue = q.borrow_mut();
            queue.push(
                Instant::now() + Duration::from_millis(delay as u64),
                JsValue::Object(obj.clone()),
            );
        }),
        _ => todo!(),
    }

    JsValue::Undefined
}
