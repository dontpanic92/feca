use crate::dom::core::{
    element::{ ElementProps, XcCoreElement},
    node::{ NodeProps, XcCoreNode, CoreNodeBase},
    Nil, XcDataType, XcObject,
};

pub fn test() {
    let obj = XcObject::new(XcCoreNode::new(
    NodeProps::new(1),
        XcCoreElement::new(ElementProps::new("a".to_string()), Nil {}),
    ));

    call(&obj);
}

fn call<T: XcDataType>(e: &CoreNodeBase<T>) {
    e.as_CoreNodeBase().test();
}
