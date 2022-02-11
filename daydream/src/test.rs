use crate::dom::core::{
    element::{ElementProps, XcCoreElement},
    node::{CoreNodeBase, NodeProps, XcCoreNode},
};
use xcdt::{Nil, XcDataType, XcObject};

pub fn test() {
    let obj = XcObject::new(XcCoreNode::new(
        NodeProps::new(1),
        XcCoreElement::new(ElementProps::new("a".to_string()), Nil {}),
    ));

    call(&obj);
}

fn call<T: XcDataType>(e: &CoreNodeBase<T>) {
    e.ext().test();
}
