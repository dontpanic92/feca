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

xcdt::declare_xcdt!(
    CoreParagraph,
    ParagraphProps,
    CoreHtmlElement,
    CoreHtmlElementBase
);

pub struct ParagraphProps {}

impl ParagraphProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_core_paragraph(children: Vec<ComRc<INode>>, id: Option<String>) -> Rc<CoreParagraph> {
    Rc::new(
        CoreParagraph::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(None, Style::default()))
            .with(ParagraphProps::new())
            .build(),
    )
}
