use maud::{Markup, html};

pub fn render(
    degree: &str,
    institution: &str,
    period: &str,
    grade: Option<&str>,
    description: &str,
) -> Markup {
    html! {
        article class="education-item" {
            div class="education-header" {
                h3 { (degree) }
                span class="education-period" { (period) }
            }

            div class="education-meta" {
                span class="education-institution" {
                    (institution)
                }

                @if let Some(grade) = grade {
                    span class="educational-separator" { "·" }
                    span class="educational-grade" {
                        "Grade: " (grade)
                    }
                }
            }

            div class="education-description" {
                @for paragraph in description.split("\n\n") {
                    p { (paragraph) }
                }
            }
        }
    }
}
