use crate::dom::Text;

use super::character_data::CoreCharacterDataBase;
use xcdt::{Nil, XcDataType};

xcdt::declare_xcdt!(CoreText, TextProps, CoreCharacterDataBase);

pub struct TextProps {}

impl<T: XcDataType> Text for CoreTextBase<T> {
    fn split_text(&self, offset: usize) -> Box<dyn Text> {
        let s = self.as_CoreCharacterDataBase().text().clone();
        todo!();
    }
}
