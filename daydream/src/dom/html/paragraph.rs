use crate::dom::{
    core::{element::ElementProps, node::NodeProps},
    Node,
};

use super::{
    html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps},
    Paragraph,
};

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

pub fn new_core_paragraph(children: Vec<Box<dyn Node>>) -> Box<CoreParagraph> {
    Box::new(
        CoreParagraph::builder()
            .with(NodeProps::new(2, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None))
            .with(ParagraphProps::new())
            .build(),
    )
}
