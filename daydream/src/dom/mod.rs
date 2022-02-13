pub mod core;
pub mod html;

pub trait Node: std::fmt::Debug {}

pub trait Element: Node {
    fn id(&self) -> Option<&str>;
}

pub trait CharacterData: Node {
    fn text(&self) -> &str;
}

pub trait Text: CharacterData {
    fn split_text(&self, offset: usize) -> Box<dyn Text>;
}
