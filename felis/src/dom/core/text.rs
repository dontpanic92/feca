use std::rc::Rc;

use xcdt::XcDataType;

use crate::{
    dom::defs::{ComObject_Text, ITextImpl},
    layout::{text::TextLayout, Layoutable},
    rendering::Renderable,
    style::{Display, Style},
};

use super::{
    character_data::{
        CharacterDataProps, CoreCharacterData, CoreCharacterDataBase, IsCoreCharacterData,
    },
    node::{NodeProps, NodeType},
};

xcdt::declare_xcdt!(
    CoreText,
    TextProps,
    CoreCharacterData,
    CoreCharacterDataBase
);

pub struct Text(pub CoreText);
ComObject_Text!(crate::dom::core::text::Text);

impl ITextImpl for CoreText {}

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
    fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer, style_computed: &Style) {
        renderer.render_text(&self.TextProps().layout, style_computed)
    }
}

impl<T: 'static + XcDataType> Layoutable for CoreTextBase<T> {
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let text = self.CharacterDataProps().text();
        let rect =
            self.TextProps()
                .layout
                .layout(pango_context, style_computed, content_boundary, text);
        rect
    }

    fn display(&self) -> Display {
        crate::style::Display::FelisText
    }
}

pub fn new_core_text(text: String) -> Rc<CoreText> {
    Rc::new(
        CoreText::builder()
            .with(NodeProps::new(NodeType::TextNode, vec![]))
            .with(CharacterDataProps::new(text))
            .with(TextProps::new())
            .build(),
    )
}
