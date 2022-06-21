use xcdt::XcDataType;

use crate::dom::{
    core::{
        element::{CoreElement, CoreElementBase, ElementProps},
        node::NodeProps,
    },
    Node,
};

use super::HtmlElement;

xcdt::declare_xcdt!(
    CoreHtmlElement,
    HtmlElementProps,
    CoreElement,
    CoreElementBase
);

pub struct HtmlElementProps {
    title: Option<String>,
}

impl HtmlElementProps {
    pub fn new(title: Option<String>) -> Self {
        Self { title }
    }
}

impl<T: 'static + XcDataType> HtmlElement for CoreHtmlElementBase<T> {
    fn title(&self) -> Option<&str> {
        todo!()
    }
}

pub fn new_core_html_element(children: Vec<Box<dyn Node>>) -> Box<CoreHtmlElement> {
    Box::new(
        CoreHtmlElement::builder()
            .with(NodeProps::new(2, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None))
            .build(),
    )
}
