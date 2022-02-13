use crate::{dom::html::HtmlDom, rendering::cairo::CairoRenderer};

#[derive(Debug)]
pub(crate) struct Page {
    dom: HtmlDom,
}

impl Page {
    pub fn new_from_html_string(html: &str) -> Self {
        let tl_dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
        let dom = HtmlDom::from_tl_dom(&tl_dom);
        Self { dom }
    }

    pub fn layout(&mut self) {}

    pub fn paint(&self, renderer: &CairoRenderer) {}
}
