use crosscom::ComRc;

use crate::{
    defs::{ComObject_HtmlDivElement, IDomString, INode},
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::Style,
};

use super::html_element::{Attributes, CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreDiv, DivProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlDivElement(pub CoreDiv);
ComObject_HtmlDivElement!(super::HtmlDivElement);

pub struct DivProps {}

impl DivProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_div(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlDivElement {
        0: CoreDiv::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, "div"))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
                attributes,
            ))
            .with(DivProps::new())
            .build(),
    })
}
