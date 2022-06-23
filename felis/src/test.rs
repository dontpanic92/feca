use crate::dom::{
    core::{
        element::{CoreElement, ElementProps},
        node::{CoreNode, NodeProps, NodeType},
    },
    Node,
};

pub fn test() {
    let n: CoreNode = CoreNode::builder().with(NodeProps::new(NodeType::ElementNode, vec![])).build();
    let e: CoreElement = CoreElement::builder()
        .with(NodeProps::new(NodeType::ElementNode, vec![]))
        .with(ElementProps::new(None))
        .build();
    // let c = (&n as &dyn NodeImpl).children();
    // let c2 = e.children();

    let x = Box::new(e);
    // x.children();
    let q = x as Box<dyn Node>;
    // let p = q as &dyn Node;
}
