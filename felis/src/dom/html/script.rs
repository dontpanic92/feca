use xcdt::XcDataType;

use crate::{
    dom::{
        core::{
            element::ElementProps,
            node::{NodeProps, NodeType},
        },
        Node,
    },
    style::Style,
};

use super::{
    html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps},
    Script,
};

xcdt::declare_xcdt!(
    CoreScript,
    ScriptProps,
    CoreHtmlElement,
    CoreHtmlElementBase
);

pub struct ScriptProps {}

impl ScriptProps {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: 'static + XcDataType> Script for CoreScriptBase<T> {
    fn text(&self) -> &str {
        todo!();
    }
}

pub fn new_core_script(children: Vec<Box<dyn Node>>) -> Box<CoreScript> {
    Box::new(
        CoreScript::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None, Style::default()))
            .with(ScriptProps::new())
            .build(),
    )
}
