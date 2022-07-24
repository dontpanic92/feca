use xcdt::XcDataType;

use super::node::{CoreNode, CoreNodeBase};
use crate::dom::defs::ICharacterDataImpl;

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
        Self {
            text: text.trim().to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl<T: 'static + XcDataType> ICharacterDataImpl for CoreCharacterDataBase<T> {
    fn text(&self) -> () {
        self.CharacterDataProps().text()
    }
}
