use std::cell::Cell;

use crate::{common::Rectangle, dom::Text, layout::text::TextLayout};

use super::{
    character_data::{
        CharacterDataImpl, CharacterDataProps, CoreCharacterData, CoreCharacterDataBase,
    },
    node::{LayoutImpl, NodeImpl, NodeProps, RenderImpl},
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

pub(crate) trait TextImpl: IsCoreText {
    fn split_text(&self, offset: usize) -> Box<dyn Text> {
        todo!();
    }
}

impl NodeImpl for CoreText {}
impl CharacterDataImpl for CoreText {}
impl TextImpl for CoreText {}

impl LayoutImpl for CoreText {
    fn layout(
        &self,
        pango_context: &pango::Context,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let rect = self
            .props()
            .layout
            .layout(pango_context, content_boundary, self.text());
        rect
    }
}

impl RenderImpl for CoreText {
    fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer) {
        self.props().layout.render(renderer.context());
    }
}

impl<T: TextImpl> Text for T {
    fn split_text(&self, offset: usize) -> Box<dyn Text> {
        TextImpl::split_text(self, offset)
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
