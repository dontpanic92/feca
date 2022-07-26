use crosscom::ComRc;

use crate::{
    defs::{ComObject_HtmlHtmlElement, IDomString, INode},
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::{Display, Style},
};

use super::html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreHtml, HtmlProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlHtmlElement(pub CoreHtml);
ComObject_HtmlHtmlElement!(super::HtmlHtmlElement);

pub struct HtmlProps;

impl HtmlProps {
    pub fn new() -> Self {
        Self
    }
}

pub fn new_core_html(children: Vec<ComRc<INode>>, id: ComRc<IDomString>) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlHtmlElement {
        0: CoreHtml::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style {
                    display: Display::Block,
                    ..Style::default()
                },
            ))
            .with(HtmlProps::new())
            .build(),
    })
}
