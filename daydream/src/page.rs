use intertrait::cast::CastRef;

use crate::{
    common::Rectangle, dom::html::HtmlDom, layout::Layoutable, rendering::cairo::CairoRenderer,
};

pub(crate) struct Page {
    dom: HtmlDom,
    pango_context: pango::Context,
}

impl Page {
    pub fn new_from_html_string(html: &str, renderer: &CairoRenderer) -> Self {
        let tl_dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
        let dom = HtmlDom::from_tl_dom(&tl_dom);
        let pango_context = pangocairo::create_context(renderer.context()).unwrap();
        Self { dom, pango_context }
    }

    pub fn layout(&mut self) {
        let root = self.dom.root().unwrap();
        root.as_layoutable().layout(
            &self.pango_context,
            Rectangle {
                top: 50,
                left: 50,
                height: 800,
                width: 600,
            },
        );
    }

    pub fn paint(&self, renderer: &CairoRenderer) {
        let root = self.dom.root().unwrap();
        root.as_renderable().paint(renderer);
    }
}
