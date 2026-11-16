mod dev_server;
mod generator;
mod pages;
mod styles;

use generator::Page;

fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let pages = [Page::new("index.html", pages::home::render)];

    generator::build("docs", &pages)?;

    if std::env::args().any(|arg| arg == "--serve") {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        runtime.block_on(dev_server::run());
    }

    Ok(())
}
