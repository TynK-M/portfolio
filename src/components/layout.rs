use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::{components::footer, components::header, styles};

pub fn render(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (title) }
                style {
                    (PreEscaped(styles::stylesheet()))
                }
            }

            body {
                (header::render())

                main {
                    (content)
                }

                (footer::render())
            }
        }
    }
}
