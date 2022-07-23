use std::rc::Rc;

use crate::{
    dom::{
        core::{
            element::ElementProps,
            node::{NodeProps, NodeType},
        },
        Node,
    },
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

pub fn new_core_body(children: Vec<Rc<dyn Node>>, id: Option<String>) -> Rc<CoreBody> {
    Rc::new(
        CoreBody::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(None, Style::default()))
            .with(BodyProps::new())
            .build(),
    )
}
