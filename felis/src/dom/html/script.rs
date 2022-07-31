use crosscom::ComRc;
use xcdt::XcDataType;

use crate::{
    defs::{ComObject_HtmlScriptElement, IDomString, IHtmlScriptElementImpl, INode, INodeImpl},
    dom::core::node::IsCoreNode,
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::Style,
};

use super::{
    html_element::{Attributes, CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps},
    HtmlDom,
};

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

impl crate::defs::INodeImpl for CoreScript {
    fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) {
        let html = "<script>".to_string() + html.str() + "</script>";
        let tl_dom = tl::parse(&html, tl::ParserOptions::default()).unwrap();
        let dom = HtmlDom::from_tl_dom(&tl_dom);
        let root = dom.root().unwrap();
        self.NodeProps().set_children(root.children());
    }
}

impl<T: 'static + XcDataType> IHtmlScriptElementImpl for CoreScriptBase<T> {
    fn text(&self) -> ComRc<IDomString> {
        self.inner_html()
    }
}

pub fn new_core_script(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    ComRc::<INode>::from_object(Script {
        0: CoreScript::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, "script"))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
                attributes,
            ))
            .with(ScriptProps::new())
            .build(),
    })
}
