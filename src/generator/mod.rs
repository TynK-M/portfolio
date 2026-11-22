mod assets;

use maud::Markup;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub struct Page {
    pub output: &'static str,
    pub content: Markup,
}

impl Page {
    pub fn new(output: &'static str, content: Markup) -> Self {
        Self { output, content }
    }
}

pub fn build(output_dir: impl AsRef<Path>, pages: Vec<Page>) -> io::Result<()> {
    let output_dir = output_dir.as_ref();

    fs::create_dir_all(output_dir)?;

    let static_dir = Path::new(crate::config::STATIC_DIR);

    if static_dir.exists() {
        assets::copy_directory(static_dir, output_dir)?;
        log::info!("Copied static assets");
    }

    log::info!("Building site into {}", output_dir.display());

    for page in pages {
        let destination: PathBuf = output_dir.join(page.output);

        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&destination, page.content.into_string())?;

        log::info!("Generated {}", destination.display());
    }

    log::info!("Site generation complete.");

    Ok(())
}
