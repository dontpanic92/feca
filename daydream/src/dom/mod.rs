pub mod core;
pub mod html;

pub trait Node {}

pub trait Element: Node {
    fn id(&self) -> String;
}

pub trait CharacterData: Node {
    fn text(&self) -> String;
}

pub trait Text: CharacterData {
    fn split_text(&self, offset: usize) -> Box<dyn Text>;
}
