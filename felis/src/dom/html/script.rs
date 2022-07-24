use std::rc::Rc;

use crosscom::ComRc;
use xcdt::XcDataType;

use crate::{
    common::Rectangle,
    dom::{
        core::{
            element::ElementProps,
            node::{NodeProps, NodeType},
        },
        defs::INode,
    },
    layout::Layoutable,
    rendering::Renderable,
    style::{Display, Style},
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

impl Renderable for CoreScript {
    fn paint(&self, _: &crate::rendering::cairo::CairoRenderer, _: &Style) {}
}

impl Layoutable for CoreScript {
    fn layout(
        &self,
        _: &pango::Context,
        _: &Style,
        _: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        Rectangle::new(0, 0, 0, 0)
    }

    fn display(&self) -> Display {
        Display::None
    }
}

pub fn new_core_script(children: Vec<ComRc<INode>>, id: Option<String>) -> Rc<CoreScript> {
    Rc::new(
        CoreScript::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(None, Style::default()))
            .with(ScriptProps::new())
            .build(),
    )
}
