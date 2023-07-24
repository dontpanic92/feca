use crosscom::ComRc;

use crate::{
    comdef::IRenderable,
    common::Rectangle,
    style::{Display, JustifyContent, Style},
};
pub struct FlowLayout;

impl FlowLayout {
    pub fn layout(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[ComRc<IRenderable>],
    ) -> Rectangle {
        if style_computed.display.unwrap() == Display::Flex {
            Self::flex(pango_context, style_computed, content_boundary, children)
        } else {
            Self::flow(pango_context, style_computed, content_boundary, children)
        }
    }

    fn flex(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[ComRc<IRenderable>],
    ) -> Rectangle {
        match style_computed.justify_content {
            Some(JustifyContent::SpaceAround) => {
                Self::flex_space_around(pango_context, style_computed, content_boundary, children)
            }
            Some(JustifyContent::SpaceBetween) => {
                Self::flex_space_between(pango_context, style_computed, content_boundary, children)
            }
            Some(JustifyContent::SpaceEvenly) => {
                Self::flex_space_evenly(pango_context, style_computed, content_boundary, children)
            }
            Some(JustifyContent::Center)
            | Some(JustifyContent::FlexStart)
            | Some(JustifyContent::FlexEnd) => Self::flex_start_end_center(
                pango_context,
                style_computed,
                content_boundary,
                children,
                style_computed.justify_content.unwrap(),
            ),
            _ => todo!(),
        }
    }

    fn flex_start_end_center(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[ComRc<IRenderable>],
        justify_content: JustifyContent,
    ) -> Rectangle {
        let width = content_boundary.width();
        let children_count = children.len();
        let max_width_per_child = width as usize / children_count;
        let total_content_width: i32 = children
            .iter()
            .map(|c| {
                c.layout(pango_context, style_computed, content_boundary)
                    .width()
            })
            .sum();

        let space_width = match justify_content {
            JustifyContent::Center => (width - total_content_width) / 2,
            JustifyContent::FlexStart
            | JustifyContent::FlexEnd
            | JustifyContent::Start
            | JustifyContent::End => width - total_content_width,
            _ => unreachable!(),
        };

        let mut next_child_boundary = content_boundary;
        next_child_boundary.width = max_width_per_child as i32;
        next_child_boundary.left += match justify_content {
            JustifyContent::Center | JustifyContent::End | JustifyContent::FlexEnd => space_width,
            JustifyContent::Start | JustifyContent::FlexStart => 0,
            _ => unreachable!(),
        };

        let mut max_child_height = 0;

        for child in children {
            let boundary = child.layout(pango_context, style_computed, next_child_boundary);
            max_child_height = if boundary.height > max_child_height {
                boundary.height
            } else {
                max_child_height
            };

            next_child_boundary.left = boundary.right();
        }

        Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: max_child_height,
            width: content_boundary.width,
        }
    }

    fn flex_space_around(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[ComRc<IRenderable>],
    ) -> Rectangle {
        let width = content_boundary.width();
        let children_count = children.len();
        let max_width_per_child = width as usize / children_count;
        let total_content_width: i32 = children
            .iter()
            .map(|c| {
                c.layout(pango_context, style_computed, content_boundary)
                    .width()
            })
            .sum();

        let space_width = (width - total_content_width) / (children_count as i32 * 2);

        let mut next_child_boundary = content_boundary;
        next_child_boundary.width = max_width_per_child as i32;

        let mut max_child_height = 0;

        for child in children {
            next_child_boundary.left += space_width;
            let boundary = child.layout(pango_context, style_computed, next_child_boundary);
            max_child_height = if boundary.height > max_child_height {
                boundary.height
            } else {
                max_child_height
            };

            next_child_boundary.left = boundary.right() + space_width;
        }

        Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: max_child_height,
            width: content_boundary.width,
        }
    }

    fn flex_space_evenly(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[ComRc<IRenderable>],
    ) -> Rectangle {
        let width = content_boundary.width();
        let children_count = children.len();
        let max_width_per_child = width as usize / children_count;
        let total_content_width: i32 = children
            .iter()
            .map(|c| {
                c.layout(pango_context, style_computed, content_boundary)
                    .width()
            })
            .sum();

        let space_width = (width - total_content_width) / (children_count as i32 * 2 + 2);

        let mut next_child_boundary = content_boundary;
        next_child_boundary.width = max_width_per_child as i32;
        next_child_boundary.left += space_width;

        let mut max_child_height = 0;

        for child in children {
            next_child_boundary.left += space_width;
            let boundary = child.layout(pango_context, style_computed, next_child_boundary);
            max_child_height = if boundary.height > max_child_height {
                boundary.height
            } else {
                max_child_height
            };

            next_child_boundary.left = boundary.right() + space_width;
        }

        Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: max_child_height,
            width: content_boundary.width,
        }
    }

    fn flex_space_between(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[ComRc<IRenderable>],
    ) -> Rectangle {
        let width = content_boundary.width();
        let children_count = children.len();
        let max_width_per_child = width as usize / children_count;
        let total_content_width: i32 = children
            .iter()
            .map(|c| {
                c.layout(pango_context, style_computed, content_boundary)
                    .width()
            })
            .sum();
        let space_width = (width - total_content_width) / (children_count as i32 - 1);

        let mut next_child_boundary = content_boundary;
        next_child_boundary.width = max_width_per_child as i32;

        let mut max_child_height = 0;

        for child in children {
            let boundary = child.layout(pango_context, style_computed, next_child_boundary);
            max_child_height = if boundary.height > max_child_height {
                boundary.height
            } else {
                max_child_height
            };

            next_child_boundary.left = boundary.right() + space_width;
        }

        Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: max_child_height,
            width: content_boundary.width,
        }
    }

    fn flow(
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
        children: &[ComRc<IRenderable>],
    ) -> Rectangle {
        let mut last_boundary = Rectangle {
            left: content_boundary.left,
            top: content_boundary.top,
            height: 0,
            width: 0,
        };

        let mut last_display = style_computed.display.unwrap();
        let mut max_right = 0;

        for child in children {
            let display = if child.display() == Display::Inherit {
                style_computed.display.unwrap()
            } else if child.display() == Display::FelisText {
                last_display
            } else {
                child.display()
            };

            let boundary = match display {
                Display::Block | Display::Flex => Rectangle::new_ltrb(
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
                Display::None => last_boundary.clone(),
                Display::Inherit | Display::FelisText => {
                    unreachable!("This branch should not be reachable")
                }
            };

            last_boundary = child.layout(pango_context, style_computed, boundary);
            last_display = display;
            max_right = if last_boundary.right() > max_right {
                last_boundary.right()
            } else {
                max_right
            };
        }

        let width = match style_computed.display {
            Some(Display::Block) => content_boundary.width,
            Some(Display::Inline) => max_right - content_boundary.left,
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
