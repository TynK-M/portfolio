use crate::components::layout;
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "Matteo Pauroso",
        html! {
            section class="hero" {
                h1 { "Hello, I'm Matteo Pauroso." }
                p { "I'm a developer who loves building things for the web." }
            }
        },
    )
}
