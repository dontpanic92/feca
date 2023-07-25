use cairo::Context;

use crate::{common::Rectangle, style::Style};

pub fn render_box(ctx: &Context, boundary: &Rectangle, style: &Style) {
    render_background(ctx, boundary, style);
    render_border(ctx, boundary, style)
}

pub fn render_background(ctx: &Context, boundary: &Rectangle, style: &Style) {
    if let Some(color) = &style.background_color {
        let color_f = color.to_color_f();
        ctx.set_source_rgba(color_f.r, color_f.g, color_f.b, color_f.a);

        ctx.rectangle(
            boundary.left as f64,
            boundary.top as f64,
            boundary.width as f64,
            boundary.height as f64,
        );
        ctx.fill().unwrap()
    }
}

pub fn render_border(ctx: &Context, boundary: &Rectangle, style: &Style) {
    println!("style: {:?}", style);
    if let Some(color) = &style.border_top_color {
        let color_f = color.to_color_f();
        ctx.set_source_rgba(color_f.r, color_f.g, color_f.b, color_f.a);
        ctx.set_line_width(1.);
        ctx.move_to(boundary.left as f64, boundary.top as f64);
        ctx.line_to(boundary.right() as f64, boundary.top as f64);
        ctx.stroke().unwrap();
    }

    if let Some(color) = &style.border_right_color {
        let color_f = color.to_color_f();
        ctx.set_source_rgba(color_f.r, color_f.g, color_f.b, color_f.a);
        ctx.set_line_width(1.);
        ctx.move_to(boundary.right() as f64, boundary.top as f64);
        ctx.line_to(boundary.right() as f64, boundary.bottom() as f64);
        ctx.stroke().unwrap();
    }

    if let Some(color) = &style.border_left_color {
        let color_f = color.to_color_f();
        ctx.set_source_rgba(color_f.r, color_f.g, color_f.b, color_f.a);
        ctx.set_line_width(1.);
        ctx.move_to(boundary.left as f64, boundary.top as f64);
        ctx.line_to(boundary.left as f64, boundary.bottom() as f64);
        ctx.stroke().unwrap();
    }

    if let Some(color) = &style.border_bottom_color {
        let color_f = color.to_color_f();
        ctx.set_source_rgba(color_f.r, color_f.g, color_f.b, color_f.a);
        ctx.set_line_width(1.);
        ctx.move_to(boundary.left as f64, boundary.bottom() as f64);
        ctx.line_to(boundary.right() as f64, boundary.bottom() as f64);
        ctx.stroke().unwrap();
    }
}
