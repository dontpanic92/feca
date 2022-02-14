use crate::{common::Rectangle, dom::Node};

use super::Layout;


pub struct TextLayout;

impl TextLayout {
    pub fn new() -> Self {
        Self {}
    }
}

impl Layout for TextLayout {
    fn layout(
        &self,
        pango_context: &pango::Context,
        content_boundary: Rectangle,
        children: &[Box<dyn Node>],
    ) -> Rectangle {
        let layout = pango::Layout::new(pango_context);
        //let text = self.text();
        layout.set_text("");
        let width = layout.height();
        let height = layout.height();

        Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height,
            width,
        }
    }
}
