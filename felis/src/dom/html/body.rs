use intertrait::castable_to;

use crate::{
    dom::{
        core::{
            element::ElementProps,
            node::{NodeProps, NodeType},
        },
        Node,
    },
    layout::Layoutable,
    style::Style,
};

use super::html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreBody, BodyProps, CoreHtmlElement, CoreHtmlElementBase);

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
            .with(HtmlElementProps::new(None, Style::default()))
            .with(BodyProps::new())
            .build(),
    )
}
