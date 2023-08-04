#![feature(let_chains)]

use url::Url;
use view::{View, ViewOptions};

mod protocols;
mod runtime;
mod view;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() < 2 {
        "./feca/tests/index.html"
        // "https://en.wikipedia.org/wiki/Web_browser"
    } else {
        &args[1]
    };

    let url = Url::parse(filename).unwrap_or_else(|_| {
        Url::from_file_path(
            std::env::current_dir()
                .unwrap()
                .join(std::path::PathBuf::from(filename))
                .canonicalize()
                .unwrap(),
        )
        .unwrap()
    });
    let html = match url.scheme() {
        "https" | "http" => protocols::http::get(&url).await?,
        "file" => protocols::file::read(&url)?,
        _ => panic!("unsupported scheme"),
    };

    let mut view = View::new(ViewOptions {
        enable_css: true,
        enable_javascript: false,
    });
    view.load_html_string(&html);

    view.run();

    Ok(())
}
