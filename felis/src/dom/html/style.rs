use crosscom::ComRc;
use xcdt::XcDataType;

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

xcdt::declare_xcdt!(CoreStyle, StyleProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlStyle(pub CoreStyle);
crate::ComObject_HtmlStyleElement!(super::HtmlStyle);

pub struct StyleProps {}

impl StyleProps {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn new_style(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlStyle {
        0: CoreStyle::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, "style"))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
                attributes,
            ))
            .with(StyleProps::new())
            .build(),
    })
}
