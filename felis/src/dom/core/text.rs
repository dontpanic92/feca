use crosscom::ComRc;

use crate::{
    defs::{ComObject_Text, IDomString, INode, IRenderableImpl, ITextImpl},
    layout::text::TextLayout,
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

impl crate::defs::INodeImpl for CoreText {
    default fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        self.CharacterDataProps().text().clone()
    }
}

impl IRenderableImpl for CoreText {
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        let text = self.CharacterDataProps().text().str();
        let rect =
            self.TextProps()
                .layout
                .layout(pango_context, style_computed, content_boundary, text);
        rect
    }

    fn display(&self) -> Display {
        crate::style::Display::FelisText
    }

    fn paint(&self, renderer: &crate::rendering::cairo::CairoRenderer, style_computed: &Style) {
        renderer.render_text(&self.TextProps().layout, style_computed)
    }
}

pub fn new_core_text(text: String) -> ComRc<INode> {
    ComRc::<INode>::from_object(Text {
        0: CoreText::builder()
            .with(NodeProps::new(NodeType::TextNode, vec![]))
            .with(CharacterDataProps::new(text))
            .with(TextProps::new())
            .build(),
    })
}
