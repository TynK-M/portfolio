use crate::data::github::Repository;
use maud::{Markup, html};

pub fn render(repository: &Repository) -> Markup {
    html! {
        article class="project-card" {
            div class="project-card-header" {
                h2 {
                    a href=(repository.html_url)
                        target="_blank"
                        rel="noopener noreferrer" {
                        img
                            src="https://cdn.simpleicons.org/github/a89984"
                            alt="GitHub"
                            width="24"
                            height="24";
                        span { (repository.name) }
                    }
                }
            }

            p class="project-card-description" {
                (repository.description.as_deref()
                    .unwrap_or("No description available."))
            }

            div class="project-card-meta" {
                @if let Some(language) = &repository.language {
                    span class="project-language" {
                        (language)
                    }
                }

                span class="project-stars" {
                    "★ " (repository.stargazers_count)
                }

                span class="project-forks" {
                    "⑂ " (repository.forks_count)
                }

                span class="project-issues" {
                    "● " (repository.open_issues_count)
                }

                span class="project-updated" {
                    "Updated " (repository.updated_at.get(..10).unwrap_or("Unknown"))
                }
            }
        }
    }
}
