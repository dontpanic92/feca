pub mod core;
pub mod html;

pub(crate) trait Node {}

pub(crate) trait Element: Node {
    fn id(&self) -> String;
}

pub(crate) trait CharacterData: Node {
    fn text(&self) -> String;
}

pub(crate) trait Text: CharacterData {
    fn split_text(&self, offset: usize) -> Box<dyn Text>;
}
