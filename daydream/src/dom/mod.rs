use intertrait::CastFrom;

use crate::{layout::Layoutable, rendering::Renderable};

pub mod core;
//pub mod html;

pub trait Node: CastFrom {
    fn children(&self) -> &[Box<dyn Node>];

    fn as_layoutable(&self) -> &dyn Layoutable;
    fn as_renderable(&self) -> &dyn Renderable;
}

pub trait Element {
    fn id(&self) -> Option<&str>;
}

pub trait CharacterData {
    fn text(&self) -> &str;
}

pub trait Text {
    fn split_text(&self, offset: usize) -> Box<dyn Text>;
}
