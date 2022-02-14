use crate::{common::Rectangle, dom::Node};

use super::Layoutable;

pub struct FlowLayout;

impl FlowLayout {
    pub fn new() -> Self {
        Self {}
    }

    pub fn layout(
        pango_context: &pango::Context,
        content_boundary: Rectangle,
        children: &[&dyn Layoutable],
    ) -> Rectangle {
        let mut next_boundary = content_boundary;
        for child in children {
            let boundary = child.layout(pango_context, next_boundary);
            next_boundary.top = boundary.top + boundary.height;
        }

        Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: next_boundary.top - content_boundary.top,
            width: content_boundary.width,
        }
    }
}
