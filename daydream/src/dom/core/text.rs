use crate::dom::Text;

use super::character_data::{CoreCharacterData, CoreCharacterDataBase};
use xcdt::XcDataType;

xcdt::declare_xcdt!(
    CoreText,
    TextProps,
    CoreCharacterData,
    CoreCharacterDataBase
);

pub struct TextProps {}

impl<T: XcDataType> Text for CoreTextBase<T> {
    fn split_text(&self, offset: usize) -> Box<dyn Text> {
        let s = self.ext().ext().text().clone();
        todo!();
    }
}
