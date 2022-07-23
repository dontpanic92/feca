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

xcdt::declare_xcdt!(CoreHead, HeadProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HeadProps {}

impl HeadProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_core_head(children: Vec<Rc<dyn Node>>, id: Option<String>) -> Rc<CoreHead> {
    Rc::new(
        CoreHead::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(None, Style::default()))
            .with(HeadProps::new())
            .build(),
    )
}
