use std::cell::{Ref, RefCell};

use pango::Layout;

use crate::{
    common::Rectangle,
    style::{FontStyle, StyleContext, TextDecorationLine},
};

pub struct TextLayout {
    layout: RefCell<Option<pango::Layout>>,
    boundary: RefCell<Rectangle>,
}

impl TextLayout {
    pub fn new() -> Self {
        Self {
            layout: RefCell::new(None),
            boundary: RefCell::new(Rectangle::new(0, 0, 0, 0)),
        }
    }

    pub fn layout(
        &self,
        pango_context: &pango::Context,
        style_context: &StyleContext,
        content_boundary: Rectangle,
        text: &str,
    ) -> Rectangle {
        let mut layout_borrow = {
            if self.layout.borrow().is_none() {
                self.layout.replace(Some(Layout::new(pango_context)));
            }

            self.layout.borrow_mut()
        };

        let layout = layout_borrow.as_mut().unwrap();

        let desc_str = style_context
            .font_family
            .as_deref()
            .or(Some("Sans Serif"))
            .unwrap();

        let mut desc = pango::FontDescription::from_string(desc_str);
        if let Some(font_style) = style_context.font_style.as_ref() {
            match font_style {
                &FontStyle::Normal => desc.set_style(pango::Style::Normal),
                &FontStyle::Italic => desc.set_style(pango::Style::Italic),
                &FontStyle::Oblique => desc.set_style(pango::Style::Oblique),
            }
        }

        println!("{}", text);
        layout.set_text(text);
        layout.set_width(content_boundary.width * pango::SCALE);
        layout.set_font_description(Some(&desc));

        if let Some(text_decoration_line) = style_context.text_decoration_line.as_ref() {
            if text_decoration_line == &TextDecorationLine::Underline {
                let attrs = pango::AttrList::new();
                attrs.insert(pango::AttrInt::new_underline(pango::Underline::SingleLine));
                layout.set_attributes(Some(&attrs));
            }
        }

        let size = layout.size();
        println!("{} {}", size.0 / pango::SCALE, size.1 / pango::SCALE);

        let boundary = Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: size.1 / pango::SCALE,
            width: size.0 / pango::SCALE,
        };

        self.boundary.replace(boundary);

        boundary
    }

    pub fn get_layout(&self) -> Ref<Option<Layout>> {
        self.layout.borrow()
    }

    pub fn get_boundary(&self) -> Ref<Rectangle> {
        self.boundary.borrow()
    }
}
