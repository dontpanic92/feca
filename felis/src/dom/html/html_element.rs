use std::{cell::RefCell, collections::HashMap};

use crosscom::ComRc;
use winit::window::CursorIcon;
use xcdt::XcDataType;

use crate::{
    common::{Color, Rectangle},
    defs::{
        ComObject_HtmlElement, IDomString, IElementImpl, IHtmlElement, IHtmlElementImpl, INode,
        INodeImpl, IRenderableImpl,
    },
    dom::core::{
        element::{CoreElement, CoreElementBase, ElementProps},
        node::{layout_children, paint_children, NodeProps, NodeType},
        string::DomString,
    },
    page::FelisAction,
    style::{Display, FontStyle, Style, TextDecorationLine},
};

xcdt::declare_xcdt!(
    CoreHtmlElement,
    HtmlElementProps,
    CoreElement,
    CoreElementBase
);

pub struct HtmlElement(pub CoreHtmlElement);
ComObject_HtmlElement!(super::HtmlElement);

pub type Attributes = HashMap<String, Option<String>>;

pub struct HtmlElementProps {
    title: ComRc<IDomString>,
    style: Style,
    boundary: RefCell<Rectangle>,
    hover: RefCell<bool>,
    attributes: Attributes,
}

impl HtmlElementProps {
    pub fn new(title: ComRc<IDomString>, default_style: Style, attributes: Attributes) -> Self {
        let inline_style = attributes
            .get("style")
            .map(|style| crate::style::parser::parse(style.as_deref().unwrap_or("")).ok())
            .flatten()
            .unwrap_or_default();
        let style = if inline_style.0.len() == 0 {
            Style::merge(&default_style, &inline_style.1)
        } else {
            default_style
        };
        Self {
            title,
            style,
            boundary: RefCell::new(Rectangle::new(0, 0, 0, 0)),
            hover: RefCell::new(false),
            attributes,
        }
    }
}

impl<T: 'static + XcDataType> IHtmlElementImpl for CoreHtmlElementBase<T> {
    fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        if self.HtmlElementProps().boundary.borrow().point_in(x, y) {
            if self.tag().str() == "a" {
                window.set_cursor_icon(CursorIcon::Hand);
                *self.HtmlElementProps().hover.borrow_mut() = true;
            } else {
                window.set_cursor_icon(CursorIcon::Arrow);
                *self.HtmlElementProps().hover.borrow_mut() = false;
            }
        } else {
            *self.HtmlElementProps().hover.borrow_mut() = false;
        }

        for i in 0..self.children().len() {
            let node = self.children().get(i);
            let element = node.query_interface::<IHtmlElement>();
            if let Some(element) = element {
                element.on_mouse_move(x, y, window);
            }
        }
    }

    fn on_mouse_click(&self) -> FelisAction {
        if *self.HtmlElementProps().hover.borrow() == true && self.tag().str() == "a" {
            if let Some(&Some(href)) = self.HtmlElementProps().attributes.get("href").as_ref() {
                return FelisAction::RequestLoadPage(href.to_string());
            }
        }

        for i in 0..self.children().len() {
            let node = self.children().get(i);
            let element = node.query_interface::<IHtmlElement>();
            if let Some(element) = element {
                let ret = element.on_mouse_click();
                if ret != FelisAction::None {
                    return ret;
                }
            }
        }

        FelisAction::None
    }
}

impl<T: 'static + XcDataType> IRenderableImpl for CoreHtmlElementBase<T> {
    default fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let style_computed = Style::merge(&self.HtmlElementProps().style, style_computed);
        *self.HtmlElementProps().boundary.borrow_mut() = layout_children(
            self.children(),
            pango_context,
            &style_computed,
            content_boundary,
        );

        self.HtmlElementProps().boundary.borrow().clone()
    }

    default fn display(&self) -> Display {
        self.HtmlElementProps().style.display
    }

    default fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_computed: &Style,
    ) {
        let style_computed = Style::merge(&self.HtmlElementProps().style, style_computed);
        paint_children(self.children(), renderer, &style_computed)
    }
}

pub fn new_core_html_element(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    tag: &str,
    style: Style,
    attributes: Attributes,
) -> ComRc<INode> {
    ComRc::<INode>::from_object(HtmlElement {
        0: CoreHtmlElement::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, tag))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                style,
                attributes,
            ))
            .build(),
    })
}

pub fn new_i_element(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    new_core_html_element(
        children,
        id,
        "i",
        Style {
            font_style: Some(FontStyle::Italic),
            display: Display::Inline,
            ..Style::default()
        },
        attributes,
    )
}

pub fn new_a_element(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    new_core_html_element(
        children,
        id,
        "a",
        Style {
            text_color: Some(Color::BLUE),
            text_decoration_line: Some(TextDecorationLine::Underline),
            display: Display::Inline,
            ..Style::default()
        },
        attributes,
    )
}

macro_rules! new_element {
    ($name: ident, $style: expr) => {
        paste::paste! {
            pub fn [<new_ $name _element>](children: Vec<ComRc<INode>>, id: ComRc<IDomString>, attributes: Attributes) -> ComRc<INode> {
                new_core_html_element(children, id, stringify!($name), $style, attributes)
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
