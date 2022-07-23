use std::rc::Rc;

use crate::{
    dom::{
        core::{
            element::ElementProps,
            node::{NodeProps, NodeType},
        },
        Node,
    },
    style::{Display, Style},
};

use super::html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreHtml, HtmlProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlProps;

impl HtmlProps {
    pub fn new() -> Self {
        Self
    }
}

pub fn new_core_html(children: Vec<Rc<dyn Node>>, id: Option<String>) -> Rc<CoreHtml> {
    Rc::new(
        CoreHtml::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(
                None,
                Style {
                    display: Display::Block,
                    ..Style::default()
                },
            ))
            .with(HtmlProps::new())
            .build(),
    )
}
