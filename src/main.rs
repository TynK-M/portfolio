mod components;
mod dev_server;
mod generator;
mod github;
mod pages;
mod styles;

use generator::Page;

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let pages = [
        Page::new("index.html", pages::home::render),
        Page::new("about/index.html", pages::about::render),
        Page::new("projects/index.html", pages::projects::render),
        Page::new("contact/index.html", pages::contact::render),
    ];

    generator::build("docs", &pages)?;

    if std::env::args().any(|arg| arg == "--serve") {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        runtime.block_on(dev_server::run());
    }

    Ok(())
}
