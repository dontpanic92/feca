use crate::dom::Node;
use xcdt::{Object, ObjectBase, XcDataType};

xcdt::declare_xcdt!(CoreNode, NodeProps, Object, ObjectBase);

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
