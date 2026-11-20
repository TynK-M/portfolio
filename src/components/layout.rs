use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::{components::footer, components::header, styles};

pub fn render(title: &str, content: Markup, page_styles: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";

                link rel="icon" type="image/x-icon" href=(crate::site::url("favicon.ico"));
                link rel="icon" type="image/svg+xml" href=(crate::site::url("/favicon.svg"));

                title { (title) }

                style {
                    (PreEscaped(styles::global()))
                    (PreEscaped(page_styles))
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
