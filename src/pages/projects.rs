use crate::{
    components::{layout, project_card},
    github,
};
use maud::{Markup, html};

const PROJECTS: &[&str] = &["zarko"];

pub fn render() -> Markup {
    let repositories =
        github::fetch_repositories(PROJECTS).expect("Failed to fetch GitHub repositories");

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
                    @for repository in &repositories {
                        (project_card::render(repository))
                    }
                }
            }
        },
        &crate::styles::projects::stylesheet(),
    )
}
