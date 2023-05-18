use crosscom::ComRc;
use winit::window::Window;

use crate::{
    common::Rectangle,
    defs::{IHtmlElement, INode, IRenderable},
    dom::core::string::DomString,
    dom::html::HtmlDom,
    rendering::cairo::CairoRenderer,
    style::{block::StyleBlock, parser::parse_style, Style},
};

#[derive(Clone, PartialEq)]
pub enum FelisAction {
    None,
    RequestLoadPage(String),
}

pub struct PageOptions {
    pub enable_css: bool,
}

pub struct Page {
    dom: HtmlDom,
    style: Style,
    styles: Vec<StyleBlock>,
    options: PageOptions,
}

impl Page {
    pub fn new_from_html_string(html: &str, options: PageOptions) -> Self {
        let tl_dom = tl::parse(html, tl::ParserOptions::default()).unwrap();
        let dom = HtmlDom::from_tl_dom(&tl_dom);
        let styles = if options.enable_css {
            Self::parse_style_block(&dom)
        } else {
            vec![]
        };

        Self {
            dom,
            style: Style::html_default(),
            styles,
            options,
        }
    }

    pub fn document(&self) -> Option<ComRc<INode>> {
        self.dom.root()
    }

    pub fn style(&mut self) {
        if let Some(root) = &self.dom.root() {
            if let Some(root) = root.query_interface::<IHtmlElement>() {
                self.style_element(root);
            }
        }
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

    pub fn render(&mut self, renderer: &CairoRenderer) {
        if self.options.enable_css {
            self.style();
        }

        self.layout(renderer.pango_context(), renderer.canvas_size());
        self.paint(renderer);
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

    fn style_element(&mut self, element: ComRc<IHtmlElement>) {
        for block in &self.styles {
            for selector in &block.selectors {
                if selector.match_element(element.clone()) {
                    element.merge_style(&block.style);
                }
            }
        }

        for child in element.children().raw() {
            if let Some(element) = child.query_interface::<IHtmlElement>() {
                self.style_element(element)
            }
        }
    }

    fn parse_style_block(dom: &HtmlDom) -> Vec<StyleBlock> {
        let style_elements = dom
            .root()
            .unwrap()
            .get_elements_by_tag_name(DomString::new("style".to_string()));

        let link_elements = dom
            .root()
            .unwrap()
            .get_elements_by_tag_name(DomString::new("link".to_string()));

        let mut blocks = vec![];
        for element in style_elements.raw() {
            let node = element.query_interface::<INode>().unwrap();
            let css_text = node.inner_html().str();
            let css_blocks = parse_style(css_text).unwrap();
            blocks.extend(css_blocks);
        }

        for element in link_elements.raw() {
            let node = element.query_interface::<IHtmlElement>().unwrap();
            let href = node.get_attribute("href").flatten();
            if let Some(href) = href {
                let content = std::fs::read_to_string(href);
                if let Ok(css_text) = content {
                    let css_blocks = parse_style(&css_text);
                    match css_blocks {
                        Ok(b) => blocks.extend(b),
                        Err(e) => e.short_print_err(),
                    }
                }
            }
        }

        blocks
    }
}
