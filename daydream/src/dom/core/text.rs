use std::cell::Cell;

use xcdt::XcDataType;

use crate::{
    common::Rectangle,
    dom::Text,
    layout::{text::TextLayout, Layoutable},
    rendering::Renderable,
};

use super::{
    character_data::{
        CharacterDataProps, CoreCharacterData, CoreCharacterDataBase,
        IsCoreCharacterData,
    },
    element::IsCoreElement,
    node::NodeProps,
};

xcdt::declare_xcdt!(
    CoreText,
    TextProps,
    CoreCharacterData,
    CoreCharacterDataBase
);

pub struct TextProps {
    layout: TextLayout,
}

impl TextProps {
    pub fn new() -> Self {
        Self {
            layout: TextLayout::new(),
        }
    }
}

impl<T: 'static + XcDataType> Renderable for CoreTextBase<T> {
    fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer) {
        self.TextProps().layout.render(renderer.context());
    }
}

impl<T: 'static + XcDataType> Layoutable for CoreTextBase<T> {
    fn layout(
        &self,
        pango_context: &pango::Context,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let text = self.CharacterDataProps().text();
        let rect = self
            .TextProps()
            .layout
            .layout(pango_context, content_boundary, text);
        rect
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
