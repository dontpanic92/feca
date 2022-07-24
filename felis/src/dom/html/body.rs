use std::rc::Rc;

use crosscom::ComRc;

use crate::{
    dom::{
        core::{
            element::ElementProps,
            node::{NodeProps, NodeType},
        },
        defs::INode,
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

pub fn new_core_body(children: Vec<ComRc<INode>>, id: Option<String>) -> Rc<CoreBody> {
    Rc::new(
        CoreBody::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(None, Style::default()))
            .with(BodyProps::new())
            .build(),
    )
}
