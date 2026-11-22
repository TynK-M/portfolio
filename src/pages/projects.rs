use crate::components::{layout, project_card};
use crate::data::github::Repository;
use maud::{Markup, html};

pub fn render(repositories: &[Repository]) -> Markup {
    layout::render(
        "Projects | TynK-M",
        html! {
            section class="projects" {
                h1 { "Projects" }

                p {
                    "A selection of things I've built."
                }

                div class="project-legend" {
                    span class="project-stars" {
                        "★ Stars"
                    }
                    span class="project-forks" {
                        "⑂ Forks"
                    }
                    span class="project-issues" {
                        "● Issues"
                    }
                }

                div class="projects-grid" {
                    @for repository in repositories {
                        (project_card::render(repository))
                    }
                }
            }
        },
        &crate::styles::pages::projects::stylesheet(),
    )
}
