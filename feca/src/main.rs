#![feature(let_chains)]
use std::fs::read_to_string;

use view::{View, ViewOptions};

mod runtime;
mod view;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() < 2 {
        "feca/tests/index.html"
    } else {
        &args[1]
    };

    let html = read_to_string(filename).unwrap();
    let mut view = View::new(ViewOptions {
        enable_css: false,
        enable_javascript: false,
    });
    view.load_html_string(&html);

    view.run();
}
