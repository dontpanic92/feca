use catus::interpreter::Interpreter;
use felis::{CairoRenderer, DomString, Page};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct View {
    event_loop: EventLoop<()>,
    window: Window,
    page: Option<Page>,
    renderer: CairoRenderer,
    interpreter: Interpreter,
}

impl View {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let renderer = CairoRenderer::new_from_winit(&window);
        let interpreter = Interpreter::new();

        Self {
            event_loop,
            window,
            page: None,
            renderer,
            interpreter,
        }
    }

    pub fn load_html_string(&mut self, html: &str) {
        let mut page = Page::new_from_html_string(html, &self.renderer);
        page.layout();

        let root = page.document().unwrap();
        let elements = root.get_elements_by_tag_name(DomString::new("script".to_string()));
        println!("len {}", elements.len());
        for i in 0..elements.len() {
            let script = catus::parser::parse(elements.get(i).inner_html().str());
            if let Ok((s, script)) = script && s.len() == 0 {
                self.interpreter.eval(&script);
            }
        }

        self.page = Some(page);
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => *control_flow = ControlFlow::Exit,
                Event::RedrawRequested(_) => {
                    if let Some(page) = &self.page {
                        page.paint(&self.renderer);
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.renderer = CairoRenderer::new_from_winit(&self.window);
                }
                _ => (),
            }
        });
    }
}
