#![feature(generic_associated_types)]
#![feature(min_specialization)]

use crate::rendering::cairo::CairoRenderer;
use page::Page;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod common;
mod defs;
mod dom;
mod layout;
mod page;
mod rendering;
mod style;
// mod test;

fn main() {
    // test::test();

    let input = r#"<html>
    <body>
        <h1>关雎</h1>
        <p>关关雎鸠，在河之洲。窈窕淑女，君子好逑。</p>
        <p>参差荇菜，左右流之。窈窕淑女，寤寐求之。</p>
        <p>求之不得，寤寐思服。悠哉悠哉，<b>辗转反侧。</b></p>
        <p><i>参差荇菜，左右采之。窈窕淑女，琴瑟友之。</i></p>
        <p><a>参差荇菜，左右芼之。<i>窈窕淑女，</i>钟鼓乐之。</a></p>
    </body></html>"#;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut renderer = CairoRenderer::new_from_winit(&window);

    let mut page = Page::new_from_html_string(input);
    page.layout(renderer.pango_context(), renderer.canvas_size());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                page.paint(&renderer);
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
