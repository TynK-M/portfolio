use maud::{Markup, html};

pub fn render(
    role: &str,
    company: &str,
    employment_type: &str,
    period: &str,
    description: &str,
) -> Markup {
    html! {
        article class="experience-item" {
            div class="experience-header" {
                h3 { (role) }
                span class="experience-period" { (period) }
            }

            div class="experience-meta" {
                span class="experience-company" {
                    (company)
                }

                span class="experience-separator" {
                    "·"
                }

                span class="experience-type" {
                    (employment_type)
                }
            }

            div class="experience-description" {
                @for paragraph in description.split("\n\n") {
                    p { (paragraph) }
                }
            }
        }
    }
}
