use crosscom::ComRc;

use crate::{
    comdef::{IDomString, INode},
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::Style,
};

use super::html_element::{Attributes, CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(
    CoreParagraph,
    ParagraphProps,
    CoreHtmlElement,
    CoreHtmlElementBase
);

pub struct HtmlParagraphElement(pub CoreParagraph);
crate::ComObject_HtmlParagraphElement!(super::HtmlParagraphElement);

pub struct ParagraphProps {}

impl ParagraphProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_p(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlParagraphElement {
        0: CoreParagraph::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, "p"))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
                attributes,
            ))
            .with(ParagraphProps::new())
            .build(),
    })
}
