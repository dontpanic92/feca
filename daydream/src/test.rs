use crate::dom::{
    core::{
        element::{CoreElement, ElementImpl, ElementProps},
        node::{CoreNode, NodeImpl, NodeProps},
    },
    Node,
};

pub fn test() {
    let n: CoreNode = CoreNode::builder().with(NodeProps::new(1, vec![])).build();
    let e: CoreElement = CoreElement::builder()
        .with(NodeProps::new(1, vec![]))
        .with(ElementProps::new(None))
        .build();
    let c = (&n as &dyn NodeImpl).children();
    // let c2 = e.children();

    let x = Box::new(e);
    // x.children();
    let q = x as Box<dyn Node>;
    // let p = q as &dyn Node;
}
