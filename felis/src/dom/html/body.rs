use crosscom::ComRc;

use crate::{
    defs::{ComObject_HtmlBodyElement, IDomString, INode},
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::Style,
};

use super::html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreBody, BodyProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlBodyElement(pub CoreBody);
ComObject_HtmlBodyElement!(super::HtmlBodyElement);

pub struct BodyProps;

impl BodyProps {
    pub fn new() -> Self {
        Self
    }
}

pub fn new_core_body(children: Vec<ComRc<INode>>, id: ComRc<IDomString>) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlBodyElement {
        0: CoreBody::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
            ))
            .with(BodyProps::new())
            .build(),
    })
}
