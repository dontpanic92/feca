use crate::{
    dom::Node,
    layout::{flow::FlowLayout, Test, Layout},
};
use xcdt::{Object, ObjectBase, XcDataType};

xcdt::declare_xcdt!(CoreNode, NodeProps, Object, ObjectBase);

pub struct NodeProps {
    node_type: i32,
    children: Vec<Box<dyn Node>>,
    layout: FlowLayout,
}

impl NodeProps {
    pub fn new(node_type: i32, children: Vec<Box<dyn Node>>) -> Self {
        Self {
            node_type,
            children,
            layout: FlowLayout {},
        }
    }
}

/*impl Test for dyn ImplTestViaNode {
    fn test(&self) {
        println!("in node test");
    }
}

pub trait ImplTestViaNode {}
impl ImplTestViaNode for CoreNode {

}*/

impl<T: XcDataType> Node for CoreNodeBase<T> {
    fn children(&self) -> &[Box<dyn Node>] {
        println!("{:?}", &self);
        self.ext().properties().children.as_ref()
    }

    /*fn layout(
        &self,
        pango_context: &pango::Context,
        boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        self.ext()
            .properties()
            .layout
            .layout(pango_context, boundary, self.children())
    }*/
}
