use crate::common::Rectangle;

pub mod box_model;
pub mod flow;
pub mod text;

pub trait Layout {
    fn layout(
        &self,
        pango_context: &pango::Context,
        content_boundary: Rectangle,
        children: &[Box<dyn crate::dom::Node>],
    ) -> Rectangle;
}

pub trait Test {
    fn test(&self);
}
