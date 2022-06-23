use intertrait::CastFrom;

use crate::{common::Rectangle, style::StyleContext};

pub mod box_model;
pub mod flow;
pub mod text;

pub trait Layoutable: CastFrom {
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_context: &StyleContext,
        content_boundary: Rectangle,
    ) -> Rectangle;
}
