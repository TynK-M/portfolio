use maud::Markup;
use std::fs;
use std::io;
use std::path::Path;

pub struct Page {
    pub output: &'static str,
    pub render: fn() -> Markup,
}

impl Page {
    pub const fn new(output: &'static str, render: fn() -> Markup) -> Self {
        Self { output, render }
    }
}

pub fn build(output_dir: impl AsRef<Path>, pages: &[Page]) -> io::Result<()> {
    let output_dir = output_dir.as_ref();

    fs::create_dir_all(output_dir)?;

    log::info!("Building site into {}", output_dir.display());

    for page in pages {
        let destination = output_dir.join(page.output);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        let html = (page.render)().into_string();

        fs::write(&destination, html)?;

        log::info!("Generated {}", destination.display());
    }

    log::info!("Site generation complete.");

    Ok(())
}
