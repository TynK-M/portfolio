mod pages;
mod styles;

use std::fs;

fn main() {
    let home = pages::home::render();

    fs::create_dir_all("docs").expect("Failed to create docs directory");

    fs::write("docs/index.html", home.into_string()).expect("Failed to write index.html");
}
