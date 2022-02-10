use crate::dom::Node;
use xcdt::{Nil, XcDataType, XcObjectBase};

xcdt::declare_xcdt!(CoreNode, NodeProps, XcObjectBase);

pub struct NodeProps {
    node_type: i32,
}

impl NodeProps {
    pub fn new(node_type: i32) -> Self {
        Self { node_type }
    }
}

impl<T: XcDataType> Node for CoreNodeBase<T> {}

impl<T: XcDataType> XcCoreNode<T> {
    pub fn test(&self) {
        println!("{}", 1);
    }
}
