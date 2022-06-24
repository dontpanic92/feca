use xcdt::XcDataType;

use crate::{
    common::Color,
    dom::{
        core::{
            element::{CoreElement, CoreElementBase, ElementProps},
            node::{layout_children, paint_children, IsCoreNode, NodeProps, NodeType},
        },
        Node,
    },
    layout::Layoutable,
    rendering::Renderable,
    style::{FontStyle, Style, StyleContext, TextDecorationLine},
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
    fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_context: &StyleContext,
    ) {
        let style_context = StyleContext::merge(style_context, &self.HtmlElementProps().style);
        paint_children(self.NodeProps().children(), renderer, &style_context)
    }
}

impl<T: 'static + XcDataType> Layoutable for CoreHtmlElementBase<T> {
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_context: &StyleContext,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let style_context = StyleContext::merge(style_context, &self.HtmlElementProps().style);
        layout_children(
            self.NodeProps().children(),
            pango_context,
            &style_context,
            content_boundary,
        )
    }
}

pub fn new_core_html_element(children: Vec<Box<dyn Node>>, style: Style) -> Box<CoreHtmlElement> {
    Box::new(
        CoreHtmlElement::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(None))
            .with(HtmlElementProps::new(None, style))
            .build(),
    )
}

pub fn new_i_element(children: Vec<Box<dyn Node>>) -> Box<CoreHtmlElement> {
    new_core_html_element(
        children,
        Style {
            font_style: Some(FontStyle::Italic),
            ..Style::default()
        },
    )
}

pub fn new_a_element(children: Vec<Box<dyn Node>>) -> Box<CoreHtmlElement> {
    new_core_html_element(
        children,
        Style {
            text_color: Some(Color::BLUE),
            text_decoration_line: Some(TextDecorationLine::Underline),
            ..Style::default()
        },
    )
}

macro_rules! new_element {
    ($name: ident, $style: expr) => {
        paste::paste! {
            pub fn [<new_ $name _element>](children: Vec<Box<dyn Node>>) -> Box<CoreHtmlElement> {
                new_core_html_element(children,$style)
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
