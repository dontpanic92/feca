use cairo::{Context, FontFace, Win32Surface};
use raw_window_handle::{HasRawWindowHandle, Win32Handle};
use winapi::shared::windef::HWND;

use crate::{common::Rectangle, layout::text::TextLayout, style::StyleContext};

pub struct CairoRenderer {
    _surface: Win32Surface,
    context: Context,
    canvas_width: f64,
    canvas_height: f64,
}

impl CairoRenderer {
    pub fn new_from_winit(window: &winit::window::Window) -> Self {
        let surface = Self::create_surface_from_winit(window);
        let context = cairo::Context::new(&surface).unwrap();
        let size = window.inner_size();

        Self {
            _surface: surface,
            context,
            canvas_width: size.width as f64,
            canvas_height: size.height as f64,
        }
    }

    pub fn render_text(&self, layout: &TextLayout, style_context: &StyleContext) {
        let boundary = layout.get_boundary();
        let layout = layout.get_layout();

        if let Some(color) = style_context.text_color.as_ref() {
            let color_f = color.to_color_f();
            self.context
                .set_source_rgba(color_f.r, color_f.g, color_f.b, color_f.a);
        }

        self.context
            .move_to(boundary.left as f64, boundary.top as f64);
        pangocairo::update_layout(&self.context, layout.as_ref().unwrap());
        pangocairo::show_layout(&self.context, layout.as_ref().unwrap());
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    fn create_surface_from_winit(window: &winit::window::Window) -> Win32Surface {
        match window.raw_window_handle() {
            raw_window_handle::RawWindowHandle::Win32(Win32Handle { hwnd, .. }) => {
                let dc = unsafe { winapi::um::winuser::GetDC(hwnd as HWND) };
                let surface = Win32Surface::create(dc).unwrap();
                unsafe { winapi::um::winuser::ReleaseDC(hwnd as HWND, dc) };
                surface
            }
            _ => unimplemented!(),
        }
    }
}
