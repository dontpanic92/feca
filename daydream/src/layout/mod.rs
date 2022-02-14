use intertrait::CastFrom;

use crate::common::Rectangle;

pub mod box_model;
pub mod flow;
pub mod text;

pub trait Layoutable: CastFrom {
    fn layout(&self, pango_context: &pango::Context, content_boundary: Rectangle) -> Rectangle;
}
