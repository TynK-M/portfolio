use crate::components::layout;
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "Projects | Matteo Pauroso",
        html! {
            section {
                h1 { "Projects" }
                p { "Here are some things I've been working on." }
            }
        },
    )
}
