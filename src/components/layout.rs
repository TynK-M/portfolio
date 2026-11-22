
use maud::{DOCTYPE, Markup, PreEscaped, html};

use crate::{components::footer, components::header, styles};

pub fn render(
    title: &str,
    description: &str,
    path: &str,
    content: Markup,
    page_styles: &str,
) -> Markup {
    let url = crate::config::url(path);

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";

                title { (title) }
                meta name="description" content=(description);
                meta name="theme-color" content="#282828";

                link rel="canonical" href=(url);

                link rel="icon" type="image/x-icon" href=(crate::config::url("favicon.ico"));
                link rel="icon" type="image/svg+xml" href=(crate::config::url("favicon.svg"));

                meta property="og:title" content=(title);
                meta property="og:description" content=(description);
                meta property="og:type" content="website";
                meta property="og:url" content=(url);

                meta name="twitter:card" content="summary";
                meta name="twitter:title" content=(title);
                meta name="twitter:description" content=(description);

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

