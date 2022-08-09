use std::io::Cursor;

use cairo::{Context, FontFace, ImageSurface, Pattern, Win32Surface};
use image::RgbaImage;
use raw_window_handle::{HasRawWindowHandle, Win32WindowHandle};
use winapi::shared::windef::HWND;

use crate::{common::Rectangle, layout::text::TextLayout, style::Style};

pub struct CairoRenderer {
    _surface: Win32Surface,
    context: Context,
    pango_context: pango::Context,
    canvas_width: f64,
    canvas_height: f64,
}

impl CairoRenderer {
    pub fn new_from_winit(window: &winit::window::Window) -> Self {
        let surface = Self::create_surface_from_winit(window);
        let context = cairo::Context::new(&surface).unwrap();
        let pango_context = pangocairo::create_context(&context).unwrap();
        let size = window.inner_size();

        Self {
            _surface: surface,
            context,
            pango_context,
            canvas_width: size.width as f64,
            canvas_height: size.height as f64,
        }
    }

    pub fn render_text(&self, layout: &TextLayout, style_computed: &Style) {
        let boundary = layout.get_boundary();
        let layout = layout.get_layout();

        self.context.save().unwrap();
        if let Some(color) = style_computed.text_color.as_ref() {
            let color_f = color.to_color_f();
            self.context
                .set_source_rgba(color_f.r, color_f.g, color_f.b, color_f.a);
        }

        self.context
            .move_to(boundary.left as f64, boundary.top as f64);
        pangocairo::update_layout(&self.context, layout.as_ref().unwrap());
        pangocairo::show_layout(&self.context, layout.as_ref().unwrap());
        self.context.restore().unwrap();
    }

    pub fn render_png(&self, boundary: &Rectangle, mut img: RgbaImage) {
        self.context.save().unwrap();

        let width = img.width();
        let height = img.height();
        let buf = img.as_mut_ptr();
        {
            let img = unsafe {
                cairo::ImageSurface::create_for_data_unsafe(
                    buf,
                    cairo::Format::ARgb32,
                    width as i32,
                    height as i32,
                    (width * 4) as i32,
                )
                .unwrap()
            };

            self.context
                .set_source_surface(&img, boundary.left as f64, boundary.top as f64)
                .unwrap();
            self.context.rectangle(
                boundary.left as f64,
                boundary.top as f64,
                boundary.width as f64,
                boundary.height as f64,
            );
            self.context.fill().unwrap();
        }

        self.context.restore().unwrap();
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn pango_context(&self) -> &pango::Context {
        &self.pango_context
    }

    pub fn canvas_size(&self) -> (i32, i32) {
        (self.canvas_width as i32, self.canvas_height as i32)
    }

    pub fn flush(&self) {
        self._surface.flush();
    }

    fn create_surface_from_winit(window: &winit::window::Window) -> Win32Surface {
        match window.raw_window_handle() {
            raw_window_handle::RawWindowHandle::Win32(Win32WindowHandle { hwnd, .. }) => {
                let dc = unsafe { winapi::um::winuser::GetDC(hwnd as HWND) };
                let surface = Win32Surface::create(dc).unwrap();
                surface
            }
            _ => unimplemented!(),
        }
    }
}
