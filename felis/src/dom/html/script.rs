use crosscom::ComRc;
use xcdt::XcDataType;

use crate::{
    defs::{ComObject_HtmlScriptElement, IDomString, IHtmlScriptElementImpl, INode},
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::Style,
};

use super::html_element::{CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(
    CoreScript,
    ScriptProps,
    CoreHtmlElement,
    CoreHtmlElementBase
);

pub struct Script(pub CoreScript);
ComObject_HtmlScriptElement!(super::Script);

pub struct ScriptProps {}

impl ScriptProps {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: 'static + XcDataType> IHtmlScriptElementImpl for CoreScriptBase<T> {
    fn text(&self) -> ComRc<IDomString> {
        todo!();
    }
}

pub fn new_core_script(children: Vec<ComRc<INode>>, id: ComRc<IDomString>) -> ComRc<INode> {
    ComRc::<INode>::from_object(Script {
        0: CoreScript::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
            ))
            .with(ScriptProps::new())
            .build(),
    })
}
