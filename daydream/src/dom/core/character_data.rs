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

impl<T: XcDataType> XcCoreCharacterData<T> {
    pub fn text(&self) -> &str {
        self.properties.text.as_str()
    }
}

impl<T: XcDataType> CharacterData for CoreCharacterDataBase<T> {
    fn text(&self) -> String {
        self.ext().ext().properties().text.clone()
    }
}
