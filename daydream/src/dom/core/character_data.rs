use crate::dom::CharacterData;

use super::{node::CoreNodeBase, Nil, XcDataType};

crate::declare_xcdt!(CoreCharacterData, CharacterDataProps, CoreNodeBase);
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
        self.as_CoreCharacterDataBase().properties.text.clone()
    }
}
