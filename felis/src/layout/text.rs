use std::cell::{Ref, RefCell};

use pango::Layout;

use crate::{
    common::Rectangle,
    style::{FontStyle, Style, TextDecorationLine},
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
        style_computed: &Style,
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

        let desc_str = style_computed
            .font_family
            .as_deref()
            .or(Some("Sans Serif"))
            .unwrap();

        let mut desc = pango::FontDescription::from_string(desc_str);
        if let Some(font_style) = style_computed.font_style.as_ref() {
            match font_style {
                &FontStyle::Normal => desc.set_style(pango::Style::Normal),
                &FontStyle::Italic => desc.set_style(pango::Style::Italic),
                &FontStyle::Oblique => desc.set_style(pango::Style::Oblique),
            }
        }

        if let Some(size) = style_computed.font_size.as_ref() {
            let size_i = Self::css_size_to_int(size).or(Some(12)).unwrap() * pango::SCALE;
            desc.set_size(size_i);
        }

        if let Some(weight) = style_computed.font_weight.as_ref() {
            let pango_weight = Self::css_weight_to_pango(weight).unwrap();
            desc.set_weight(pango_weight);
        }

        println!("{:?}", text);
        layout.set_text(text);
        layout.set_width(content_boundary.width * pango::SCALE);
        layout.set_font_description(Some(&desc));

        if let Some(text_decoration_line) = style_computed.text_decoration_line.as_ref() {
            if text_decoration_line == &TextDecorationLine::Underline {
                let attrs = pango::AttrList::new();
                attrs.insert(pango::AttrInt::new_underline(pango::Underline::SingleLine));
                layout.set_attributes(Some(&attrs));
            }
        }

        let size = layout.size();

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

    fn css_size_to_int(size: &str) -> Option<i32> {
        size.replace("px", "").parse().ok()
    }

    fn css_weight_to_pango(weight: &str) -> Option<pango::Weight> {
        match weight {
            "normal" | "400" => Some(pango::Weight::Normal),
            "bold" | "700" => Some(pango::Weight::Bold),
            "bolder" | "900" => Some(pango::Weight::Ultrabold),
            "lighter" | "200" => Some(pango::Weight::Light),
            _ => Some(pango::Weight::Normal),
        }
    }
}
