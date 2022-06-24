use crate::{
    common::Rectangle,
    style::{Display, Style},
};

use super::Layoutable;

pub struct FlowLayout;

impl FlowLayout {
    pub fn new() -> Self {
        Self {}
    }

    pub fn layout(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[&dyn Layoutable],
    ) -> Rectangle {
        let mut last_boundary = Rectangle {
            left: content_boundary.left,
            top: content_boundary.top,
            height: 0,
            width: 0,
        };

        let mut last_display = style_computed.display;
        let mut max_right = 0;

        for child in children {
            let display = if child.display() == Display::Inherit {
                style_computed.display
            } else if child.display() == Display::FelisText {
                last_display
            } else {
                child.display()
            };

            let boundary = match display {
                Display::Block => Rectangle::new_ltrb(
                    content_boundary.left(),
                    last_boundary.top() + last_boundary.height(),
                    content_boundary.right(),
                    content_boundary.bottom(),
                ),
                Display::Inline => Rectangle::new_ltrb(
                    last_boundary.right(),
                    last_boundary.top(),
                    content_boundary.right(),
                    last_boundary.bottom(),
                ),
                Display::Inherit | Display::FelisText => {
                    unreachable!("This branch should not be reachable")
                }
            };

            last_boundary = child.layout(pango_context, style_computed, boundary);
            last_display = child.display();
            max_right = if last_boundary.right() > max_right {
                last_boundary.right()
            } else {
                max_right
            }
        }

        let width = match style_computed.display {
            Display::Block => content_boundary.width,
            Display::Inline => max_right - content_boundary.left,
            _ => content_boundary.width,
        };

        Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: last_boundary.bottom() - content_boundary.top,
            width,
        }
    }
}
