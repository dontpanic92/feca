use crate::common::Rectangle;

pub mod core;
pub mod html;

pub trait Node {
    fn children(&self) -> &[Box<dyn Node>];

    // fn layout(&self, pango_context: &pango::Context, boundary: Rectangle) -> Rectangle;
}

pub trait Element: Node {
    fn id(&self) -> Option<&str>;
}

pub trait CharacterData: Node {
    fn text(&self) -> &str;
}

pub trait Text: CharacterData {
    fn split_text(&self, offset: usize) -> Box<dyn Text>;
}
