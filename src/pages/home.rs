use maud::{DOCTYPE, Markup, PreEscaped, html};

pub fn render() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Matteo Pauroso" };
                style {
                    (PreEscaped(crate::styles::stylesheet()))
                }
            }

            body {
                main {
                    section class="hero" {
                        h1 { "Hello, I'm Matteo Pauroso." }
                        p { "I'm a developer who loves building things for the web." }
                    }

                    section id="about" {
                        h2 { "About me" }
                        p { "A little introduction about myself." }
                    }

                    section id="projects" {
                        h2 { "Projects" }
                        p { "Here are some things I've been working on." }
                    }
                }

                footer {
                    p { "© 2026 Matteo Pauroso" }
                }
            }
        }
    }
}
