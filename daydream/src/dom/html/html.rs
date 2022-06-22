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

xcdt::declare_xcdt!(CoreHtml, HtmlProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlProps;

impl HtmlProps {
    pub fn new() -> Self {
        Self
    }
}

pub fn new_core_html(children: Vec<Box<dyn Node>>) -> Box<CoreHtml> {
    Box::new(
        CoreHtml::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None))
            .with(HtmlProps::new())
            .build(),
    )
}