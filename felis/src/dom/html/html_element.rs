use std::rc::Rc;

use crosscom::ComRc;
use xcdt::XcDataType;

use crate::{
    common::Color,
    dom::{
        core::{
            element::{CoreElement, CoreElementBase, ElementProps},
            node::{layout_children, paint_children, IsCoreNode, NodeProps, NodeType},
        },
        defs::INode,
    },
    layout::Layoutable,
    rendering::Renderable,
    style::{Display, FontStyle, Style, TextDecorationLine},
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
    style: Style,
}

impl HtmlElementProps {
    pub fn new(title: Option<String>, style: Style) -> Self {
        Self { title, style }
    }
}

impl<T: 'static + XcDataType> HtmlElement for CoreHtmlElementBase<T> {
    fn title(&self) -> Option<&str> {
        self.HtmlElementProps().title.as_deref()
    }
}

impl<T: 'static + XcDataType> Renderable for CoreHtmlElementBase<T> {
    default fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_computed: &Style,
    ) {
        let style = Style::merge(&self.HtmlElementProps().style, style_computed);
        paint_children(self.NodeProps().children(), renderer, &style)
    }
}

impl<T: 'static + XcDataType> Layoutable for CoreHtmlElementBase<T> {
    default fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let style_computed = Style::merge(&self.HtmlElementProps().style, style_computed);
        layout_children(
            self.NodeProps().children(),
            pango_context,
            &style_computed,
            content_boundary,
        )
    }

    default fn display(&self) -> Display {
        self.HtmlElementProps().style.display
    }
}

pub fn new_core_html_element(
    children: Vec<ComRc<INode>>,
    id: Option<String>,
    style: Style,
) -> Rc<CoreHtmlElement> {
    Rc::new(
        CoreHtmlElement::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id))
            .with(HtmlElementProps::new(None, style))
            .build(),
    )
}

pub fn new_i_element(children: Vec<ComRc<INode>>, id: Option<String>) -> Rc<CoreHtmlElement> {
    new_core_html_element(
        children,
        id,
        Style {
            font_style: Some(FontStyle::Italic),
            display: Display::Inline,
            ..Style::default()
        },
    )
}

pub fn new_a_element(children: Vec<ComRc<INode>>, id: Option<String>) -> Rc<CoreHtmlElement> {
    new_core_html_element(
        children,
        id,
        Style {
            text_color: Some(Color::BLUE),
            text_decoration_line: Some(TextDecorationLine::Underline),
            display: Display::Inline,
            ..Style::default()
        },
    )
}

macro_rules! new_element {
    ($name: ident, $style: expr) => {
        paste::paste! {
            pub fn [<new_ $name _element>](children: Vec<ComRc<INode>>, id: Option<String>) -> Rc<CoreHtmlElement> {
                new_core_html_element(children, id, $style)
            }
        }
    };
}

new_element!(
    h1,
    Style {
        font_size: Some("20px".to_string()),
        font_weight: Some("700".to_string()),
        ..Style::default()
    }
);

new_element!(
    b,
    Style {
        font_weight: Some("700".to_string()),
        display: Display::Inline,
        ..Style::default()
    }
);
