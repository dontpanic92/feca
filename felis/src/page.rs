use crosscom::ComRc;

use crate::{
    common::Rectangle,
    defs::{INode, IRenderable},
    dom::html::HtmlDom,
    rendering::cairo::CairoRenderer,
    style::Style,
};

pub struct Page {
    dom: HtmlDom,
    pango_context: pango::Context,
    style: Style,
}

impl Page {
    pub fn new_from_html_string(html: &str, renderer: &CairoRenderer) -> Self {
        let tl_dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
        let dom = HtmlDom::from_tl_dom(&tl_dom);
        let pango_context = pangocairo::create_context(renderer.context()).unwrap();
        Self {
            dom,
            pango_context,
            style: Style::html_default(),
        }
    }

    pub fn document(&self) -> Option<ComRc<INode>> {
        self.dom.root()
    }

    pub fn layout(&mut self) {
        let root = self.dom.root().unwrap();
        root.query_interface::<IRenderable>().unwrap().layout(
            &self.pango_context,
            &self.style,
            Rectangle {
                top: 8,
                left: 8,
                height: 800,
                width: 600,
            },
        );
    }

    pub fn paint(&self, renderer: &CairoRenderer) {
        let root = self.dom.root().unwrap();
        root.query_interface::<IRenderable>()
            .unwrap()
            .paint(renderer, &self.style);
    }
}
