use std::time::Instant;

use catus::interpreter::Interpreter;
use felis::{CairoRenderer, DomString, FelisAction, Page};
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

use crate::runtime::{setup_js_runtime, timer_queue::TIMER_QUEUE};

#[derive(Debug)]
pub enum FecaEvent {
    RequestLoadPage(String),
}

pub struct View {
    event_loop: Option<EventLoop<FecaEvent>>,
    window: Window,
    page: Option<Page>,
    renderer: CairoRenderer,
    interpreter: Option<Interpreter>,
}

impl View {
    pub fn new() -> Self {
        let event_loop = Some(EventLoopBuilder::<FecaEvent>::with_user_event().build());
        let window = WindowBuilder::new()
            .with_title("Feca")
            .build(event_loop.as_ref().unwrap())
            .unwrap();
        let renderer = CairoRenderer::new_from_winit(&window);

        Self {
            event_loop,
            window,
            page: None,
            renderer,
            interpreter: None,
        }
    }

    pub fn load_html_string(&mut self, html: &str) {
        let page = Page::new_from_html_string(html);
        let document = page.document().unwrap();
        let mut interpreter = Interpreter::new();

        setup_js_runtime(&mut interpreter, document.clone());

        let elements = document.get_elements_by_tag_name(DomString::new("script".to_string()));
        for i in 0..elements.len() {
            let script = catus::parser::parse(elements.get(i).inner_html().str());
            if let Ok((text, s)) = &script && text.len() == 0 {
                interpreter.eval(s);
            } else {
                println!("Unable to parse js: {:?}", script);
            }
        }

        self.page = Some(page);
        self.interpreter = Some(interpreter);
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap();
        let proxy = event_loop.create_proxy();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            self.handle_timer(control_flow);

            match event {
                Event::UserEvent(FecaEvent::RequestLoadPage(uri)) => {
                    let content = std::fs::read_to_string(uri).unwrap();
                    self.load_html_string(&content);
                    self.window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == self.window.id() => *control_flow = ControlFlow::Exit,
                Event::RedrawRequested(_) => {
                    if let Some(page) = &mut self.page {
                        self.renderer = CairoRenderer::new_from_winit(&self.window);
                        page.render(&self.renderer);
                        self.renderer.flush();
                    }
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(_),
                    ..
                } => {
                    self.renderer = CairoRenderer::new_from_winit(&self.window);
                }
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position, .. },
                    ..
                } => {
                    if let Some(page) = &mut self.page {
                        page.on_mouse_move(position.x, position.y, &self.window);
                    }
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::MouseInput {
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    if let Some(page) = &mut self.page {
                        let action = page.on_mouse_click();
                        match action {
                            FelisAction::None => {}
                            FelisAction::RequestLoadPage(uri) => {
                                proxy.send_event(FecaEvent::RequestLoadPage(uri)).unwrap();
                            }
                        }
                    }
                }
                _ => (),
            }
        });
    }

    fn handle_timer(&mut self, control_flow: &mut ControlFlow) {
        TIMER_QUEUE.with(|q| {
            let now = Instant::now();
            loop {
                let timeout = q.borrow().peek();
                if let Some(timeout) = timeout {
                    if now > timeout {
                        let value = q.borrow_mut().pop().unwrap();
                        self.interpreter.as_mut().unwrap().call(value);
                        self.window.request_redraw();
                        continue;
                    }
                }

                break;
            }

            if let Some(timeout) = q.borrow().peek() {
                *control_flow = ControlFlow::WaitUntil(timeout);
            }
        });
    }
}
