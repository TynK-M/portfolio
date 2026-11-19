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

fn copy_directory(source: &Path, destination: &Path) -> io::Result<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_directory(&source_path, &destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

pub fn build(output_dir: impl AsRef<Path>, pages: &[Page]) -> io::Result<()> {
    let output_dir = output_dir.as_ref();

    fs::create_dir_all(output_dir)?;

    let static_dir = Path::new("static");
    if static_dir.exists() {
        copy_directory(static_dir, output_dir)?;
        log::info!("Copied static assets");
    }

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
