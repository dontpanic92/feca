use crate::style::Style;

use self::cairo::CairoRenderer;

pub mod cairo;

pub trait Renderable {
    fn paint(&self, renderer: &CairoRenderer, style_computed: &Style);
}
