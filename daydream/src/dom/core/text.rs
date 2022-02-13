use xcdt::XcDataType;

use crate::dom::{CharacterData, Text};

use super::{
    character_data::{CharacterDataProps, CoreCharacterData, CoreCharacterDataBase},
    node::NodeProps,
};

xcdt::declare_xcdt!(
    CoreText,
    TextProps,
    CoreCharacterData,
    CoreCharacterDataBase
);

pub struct TextProps {}
impl TextProps {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: XcDataType> Text for CoreTextBase<T> {
    fn split_text(&self, offset: usize) -> Box<dyn Text> {
        let s = self.text().clone();
        todo!();
    }
}

pub fn new_core_text(text: String) -> Box<CoreText> {
    Box::new(
        CoreText::builder()
            .with(NodeProps::new(1, vec![]))
            .with(CharacterDataProps::new(text))
            .with(TextProps::new())
            .build(),
    )
}
