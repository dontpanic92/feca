use super::node::{CoreNode, CoreNodeBase, LayoutImpl, NodeImpl};
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
        Self {
            text: text.trim().to_string(),
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

pub(crate) trait CharacterDataImpl: IsCoreCharacterData {
    fn text(&self) -> &str {
        IsCoreCharacterData::props(self).text()
    }
}

impl NodeImpl for CoreCharacterData {}
impl LayoutImpl for CoreCharacterData {}
impl CharacterDataImpl for CoreCharacterData {}

impl<T: CharacterDataImpl> CharacterData for T {
    fn text(&self) -> &str {
        CharacterDataImpl::text(self)
    }
}
