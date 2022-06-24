use intertrait::CastFrom;

use crate::{
    common::Rectangle,
    style::{Display, Style},
};

pub mod box_model;
pub mod flow;
pub mod text;

pub trait Layoutable: CastFrom {
    fn display(&self) -> Display;
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: Rectangle,
    ) -> Rectangle;
}
