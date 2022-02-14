use std::cell::Cell;

use crate::common::Rectangle;

pub struct TextLayout {
    layout: Cell<Option<pango::Layout>>,
    boundary: Cell<Rectangle>,
}

thread_local! {
    pub static COUNT: Cell<u32> = Cell::new(1);
    pub static COUNT2: Cell<u32> = Cell::new(1);
}

impl TextLayout {
    pub fn new() -> Self {
        Self {
            layout: Cell::new(None),
            boundary: Cell::new(Rectangle::new(0, 0, 0, 0)),
        }
    }

    pub fn layout(
        &self,
        pango_context: &pango::Context,
        content_boundary: Rectangle,
        text: &str,
    ) -> Rectangle {
        let count = COUNT.with(|c| {
            let count = c.take();
            c.set(count + 1);
            count
        });

        let mut layout_option = self.layout.take();
        let layout = if layout_option.is_none() {
            pango::Layout::new(pango_context)
        } else {
            layout_option.take().unwrap()
        };

        let desc_str = if count == 8 {
            "Microsoft Yahei Italic"
        } else {
            "Microsoft Yahei"
        };

        let desc = pango::FontDescription::from_string(desc_str);
        println!("{}", text);
        layout.set_text(text);
        layout.set_width(content_boundary.width * pango::SCALE);
        layout.set_font_description(Some(&desc));

        if count == 10 {
            let attrs = pango::AttrList::new();
            attrs.insert(pango::AttrInt::new_underline(pango::Underline::SingleLine));
            layout.set_attributes(Some(&attrs));
        }

        let size = layout.size();
        println!("{} {}", size.0 / pango::SCALE, size.1 / pango::SCALE);

        let boundary = Rectangle {
            top: content_boundary.top,
            left: content_boundary.left,
            height: size.1 / pango::SCALE,
            width: size.0 / pango::SCALE,
        };

        self.layout.set(Some(layout));
        self.boundary.set(boundary);

        boundary
    }

    pub fn render(&self, cr: &cairo::Context) {
        let count = COUNT2.with(|c| {
            let count = c.take();
            c.set(count + 1);
            count
        });

        let boundary = self.boundary.take();
        let layout = self.layout.take().unwrap();
        cr.move_to(boundary.left as f64, boundary.top as f64);
        if count == 10 {
            cr.set_source_rgb(0., 0., 1.);
        }

        pangocairo::update_layout(cr, &layout);
        pangocairo::show_layout(cr, &layout);

        self.boundary.set(boundary);
        self.layout.set(Some(layout));
    }
}
