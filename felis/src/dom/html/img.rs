use crosscom::ComRc;
use image::RgbaImage;

use crate::dom::html::html_element::IsCoreHtmlElement;
use crate::{
    comdef::{IDomString, INode, IRenderableImpl},
    common::Rectangle,
    dom::core::{
        element::ElementProps,
        node::{NodeProps, NodeType},
        string::DomString,
    },
    style::{Display, Style},
};

use super::html_element::{Attributes, CoreHtmlElement, CoreHtmlElementBase, HtmlElementProps};

xcdt::declare_xcdt!(CoreImage, ImageProps, CoreHtmlElement, CoreHtmlElementBase);

pub struct HtmlImageElement(pub CoreImage);
crate::ComObject_HtmlImageElement!(super::HtmlImageElement);

pub struct ImageProps {
    img: Option<RgbaImage>,
}

impl ImageProps {
    pub fn new(path: Option<&str>) -> Self {
        let img = path
            .map(|p| std::fs::read(p).ok())
            .flatten()
            .map(|bytes| image::load_from_memory(&bytes).unwrap().to_rgba8());

        Self { img }
    }
}

impl IRenderableImpl for CoreImage {
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let boundary = Rectangle::new(content_boundary.top, content_boundary.left, 203, 400);
        self.HtmlElementProps().set_boundary(boundary.clone());
        boundary
    }

    fn display(&self) -> Display {
        crate::style::Display::Block
    }

    fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer, style_computed: &Style) {
        if let Some(img) = &self.ImageProps().img {
            renderer.render_png(&self.HtmlElementProps().boundary(), img.clone());
        }
    }
}

pub fn new_img(
    children: Vec<ComRc<INode>>,
    id: ComRc<IDomString>,
    attributes: Attributes,
) -> ComRc<INode> {
    let src = attributes.get("src").map(|s| s.clone()).flatten();
    ComRc::<INode>::from_object(HtmlImageElement {
        0: CoreImage::builder()
            .with(NodeProps::new(NodeType::ElementNode, children))
            .with(ElementProps::new(id, "img"))
            .with(HtmlElementProps::new(
                DomString::new("".to_string()),
                Style::default(),
                attributes,
            ))
            .with(ImageProps::new(src.as_deref()))
            .build(),
    })
}
