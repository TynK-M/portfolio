mod app;
mod components;
mod config;
mod content;
mod data;
mod generator;
mod models;
mod pages;
mod server;
mod styles;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    app::build()?;

    if std::env::args().any(|arg| arg == "--serve") {
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

        runtime.block_on(server::run());
    }

    Ok(())
}
