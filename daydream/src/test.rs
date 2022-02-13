use xcdt::XcDataType;

use crate::dom::core::{
    element::{CoreElement, ElementProps},
    node::{CoreNodeBase, NodeProps},
};

pub fn test() {
    let obj = CoreElement::builder()
        .with(NodeProps::new(1, vec![]))
        .with(ElementProps::new(None))
        .build();

    call(&obj);
}

fn call<T: XcDataType>(e: &CoreNodeBase<T>) {
    e.ext().test();
}
