use crate::{
    config, content,
    data::github,
    generator::{self, Page},
    pages,
};

use std::error::Error;

pub fn build() -> Result<(), Box<dyn Error>> {
    let repositories = github::fetch_repositories(content::projects::PROJECTS)?;

    let pages = vec![
        Page::new("index.html", pages::home::render()),
        Page::new("about/index.html", pages::about::render()),
        Page::new(
            "projects/index.html",
            pages::projects::render(&repositories),
        ),
        Page::new("contact/index.html", pages::contact::render()),
    ];

    generator::build(config::OUTPUT_DIR, pages)?;

    Ok(())
}
