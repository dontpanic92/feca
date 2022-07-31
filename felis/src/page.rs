use crosscom::ComRc;
use winit::window::Window;

use crate::{
    common::Rectangle,
    defs::{IHtmlElement, INode, IRenderable},
    dom::html::HtmlDom,
    rendering::cairo::CairoRenderer,
    style::Style,
};

#[derive(Clone, PartialEq)]
pub enum FelisAction {
    None,
    RequestLoadPage(String),
}

pub struct Page {
    dom: HtmlDom,
    style: Style,
}

impl Page {
    pub fn new_from_html_string(html: &str) -> Self {
        let tl_dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
        let dom = HtmlDom::from_tl_dom(&tl_dom);
        Self {
            dom,
            style: Style::html_default(),
        }
    }

    pub fn document(&self) -> Option<ComRc<INode>> {
        self.dom.root()
    }

    pub fn layout(&mut self, pango_context: &pango::Context, canvas_size: (i32, i32)) {
        let root = self.dom.root().unwrap();
        root.query_interface::<IRenderable>().unwrap().layout(
            pango_context,
            &self.style,
            Rectangle {
                top: 8,
                left: 8,
                height: canvas_size.1 - 8,
                width: canvas_size.0 - 8,
            },
        );
    }

    pub fn paint(&self, renderer: &CairoRenderer) {
        renderer.context().set_source_rgb(1., 1., 1.);
        renderer.context().paint().unwrap();

        let root = self.dom.root().unwrap();
        root.query_interface::<IRenderable>()
            .unwrap()
            .paint(renderer, &self.style);
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &Window) {
        if let Some(element) = self.dom.root().unwrap().query_interface::<IHtmlElement>() {
            element.on_mouse_move(x, y, window);
        }
    }

    pub fn on_mouse_click(&self) -> FelisAction {
        if let Some(element) = self.dom.root().unwrap().query_interface::<IHtmlElement>() {
            element.on_mouse_click()
        } else {
            FelisAction::None
        }
    }
}
