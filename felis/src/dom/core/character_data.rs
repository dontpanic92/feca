use crosscom::ComRc;
use xcdt::XcDataType;

use super::{
    node::{CoreNode, CoreNodeBase},
    string::DomString,
};
use crate::comdef::{ICharacterDataImpl, IDomString};

xcdt::declare_xcdt!(
    CoreCharacterData,
    CharacterDataProps,
    CoreNode,
    CoreNodeBase
);

pub struct CharacterDataProps {
    text: ComRc<IDomString>,
}

impl CharacterDataProps {
    pub fn new(text: String) -> Self {
        Self {
            text: DomString::new(text.trim().to_string()),
        }
    }

    pub fn text(&self) -> ComRc<IDomString> {
        self.text.clone()
    }
}

impl<T: 'static + XcDataType> ICharacterDataImpl for CoreCharacterDataBase<T> {
    fn text(&self) -> ComRc<IDomString> {
        self.CharacterDataProps().text()
    }
}
