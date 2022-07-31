use crosscom::ComRc;

use crate::{
    defs::{ComObject_HtmlHeadElement, IDomString, INode},
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::Style,
};

use super::html_element::{Attributes, CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreHead, HeadProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlHeadElement(pub CoreHead);
ComObject_HtmlHeadElement!(super::HtmlHeadElement);

pub struct HeadProps {}

impl HeadProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_core_head(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlHeadElement {
        0: CoreHead::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, "head"))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
                attributes,
            ))
            .with(HeadProps::new())
            .build(),
    })
}
