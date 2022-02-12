#![feature(generic_associated_types)]

use page::Page;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::rendering::cairo::CairoRenderer;
mod dom;
mod page;
mod rendering;
mod test;

fn main() {
    test::test();
    let input = r#"<body><p id="text">Hello world <strong>x</strong>good</p><p>good</p></body>"#;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let page = Page::new_from_html_string(input);
    let mut renderer = CairoRenderer::new_from_winit(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                renderer.paint();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                renderer = CairoRenderer::new_from_winit(&window);
            }
            _ => (),
        }
    });
}
