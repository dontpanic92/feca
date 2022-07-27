use std::fs::read_to_string;

use view::View;

mod runtime;
mod view;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("{} page.html", args[0]);
        return;
    }

    let html = read_to_string(&args[1]).unwrap();
    let mut view = View::new();
    view.load_html_string(&html);

    view.run();
}
