use xcdt::XcDataType;

use super::node::{CoreNode, CoreNodeBase};
use crate::dom::CharacterData;

xcdt::declare_xcdt!(
    CoreCharacterData,
    CharacterDataProps,
    CoreNode,
    CoreNodeBase
);

pub struct CharacterDataProps {
    text: String,
}

impl CharacterDataProps {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl<T: XcDataType> CharacterData for CoreCharacterDataBase<T> {
    fn text(&self) -> &str {
        self.ext().ext().properties().text.as_str()
    }
}
