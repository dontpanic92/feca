use std::fs::read_to_string;

use url::Url;

pub fn read(url: &Url) -> anyhow::Result<String> {
    let path = url.path();

    #[cfg(target_os = "windows")]
    let path = path.strip_prefix("/").unwrap();

    println!("path {}", path);
    Ok(read_to_string(path)?)
}
