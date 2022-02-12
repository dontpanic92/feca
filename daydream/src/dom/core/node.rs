use crate::dom::Node;
use xcdt::{Object, XcDataType, XcObjectBase};

xcdt::declare_xcdt!(CoreNode, NodeProps, Object, XcObjectBase);

pub struct NodeProps {
    node_type: i32,
    children: Vec<Box<dyn Node>>,
}

impl NodeProps {
    pub fn new(node_type: i32, children: Vec<Box<dyn Node>>) -> Self {
        Self {
            node_type,
            children,
        }
    }
}

impl<T: XcDataType> Node for CoreNodeBase<T> {}

impl<T: XcDataType> XcCoreNode<T> {
    pub fn test(&self) {
        println!("{}", 1);
    }
}
