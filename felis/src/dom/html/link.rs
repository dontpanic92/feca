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

xcdt::declare_xcdt!(CoreLink, LinkProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlLinkElement(pub CoreLink);
crate::ComObject_HtmlLinkElement!(super::HtmlLinkElement);

pub struct LinkProps {}

impl LinkProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_link(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlLinkElement {
        0: CoreLink::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, "link"))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
                attributes,
            ))
            .with(LinkProps::new())
            .build(),
    })
}
