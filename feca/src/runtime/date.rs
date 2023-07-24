use catus::{
    builtins::Function,
    symtbl::{JsValue, NativeFunctionProxy, Symbol},
};

pub fn make_date(_object: Symbol, function: Symbol) -> Symbol {
    Function::new_native_proxy_function(
        function,
        "Date".to_string(),
        NativeFunctionProxy::new(date),
    )
}

fn date(_params: &[JsValue]) -> JsValue {
    JsValue::String(
        chrono::Local::now()
            .format("%a %b %e %Y %T GMT%z (%Z)")
            .to_string(),
    )
}
