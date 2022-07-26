use crosscom::ComRc;

use crate::{
    defs::{ComObject_HtmlParagraphElement, IDomString, INode},
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
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

pub struct HtmlParagraphElement(pub CoreParagraph);
ComObject_HtmlParagraphElement!(super::HtmlParagraphElement);

pub struct ParagraphProps {}

impl ParagraphProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_core_paragraph(children: Vec<ComRc<INode>>, id: ComRc<IDomString>) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlParagraphElement {
        0: CoreParagraph::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
            ))
            .with(ParagraphProps::new())
            .build(),
    })
}
