use cairo::{Context, FontFace, Win32Surface};
use raw_window_handle::{HasRawWindowHandle, Win32Handle};
use winapi::shared::windef::HWND;

pub(crate) struct CairoRenderer {
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

    pub fn paint(&self) {
        self.context.set_source_rgb(1., 1., 1.);
        self.context
            .rectangle(0., 0., self.canvas_width, self.canvas_height);
        self.context.fill().unwrap();

        let face =
            FontFace::toy_create("宋体", cairo::FontSlant::Normal, cairo::FontWeight::Normal)
                .unwrap();
        self.context.set_font_face(&face);
        self.context.set_font_size(80.);
        self.context.move_to(0., 100.);
        self.context.set_source_rgb(0., 0., 0.);
        self.context.show_text("test").unwrap();
        // self.context.paint().unwrap();
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
