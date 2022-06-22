use intertrait::castable_to;
use xcdt::XcDataType;

use crate::{
    dom::{
        core::{element::ElementProps, node::{NodeProps, NodeType}},
        Node,
    },
    layout::Layoutable,
};

use super::html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreBody, BodyProps, CoreHtmlElement);

pub struct BodyProps;

impl BodyProps {
    pub fn new() -> Self {
        Self
    }
}

castable_to!(CoreBody => Node, Layoutable);

pub fn new_core_body(children: Vec<Box<dyn Node>>) -> Box<CoreBody> {
    Box::new(
        CoreBody::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None))
            .with(BodyProps::new())
            .build(),
    )
}
