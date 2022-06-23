use xcdt::XcDataType;

use crate::{
    layout::{text::TextLayout, Layoutable},
    rendering::Renderable,
    style::StyleContext,
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
    fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_context: &StyleContext,
    ) {
        renderer.render_text(&self.TextProps().layout, style_context)
    }
}

impl<T: 'static + XcDataType> Layoutable for CoreTextBase<T> {
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_context: &StyleContext,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let text = self.CharacterDataProps().text();
        let rect =
            self.TextProps()
                .layout
                .layout(pango_context, style_context, content_boundary, text);
        rect
    }
}

pub fn new_core_text(text: String) -> Box<CoreText> {
    Box::new(
        CoreText::builder()
            .with(NodeProps::new(NodeType::TextNode, vec![]))
            .with(CharacterDataProps::new(text))
            .with(TextProps::new())
            .build(),
    )
}
