use crate::dom::{
    core::{
        element::{CoreElement, CoreElementBase, ElementProps},
        node::{NodeImpl, NodeProps},
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

pub(crate) trait HtmlElementImpl: IsCoreHtmlElement {}

impl NodeImpl for CoreHtmlElement {}
impl HtmlElementImpl for CoreHtmlElement {}

impl<T: HtmlElementImpl> HtmlElement for T {
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
