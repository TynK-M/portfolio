use crate::components::layout;
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "Matteo Pauroso",
        html! {
            section class="hero" {
                h1 { "Hello, I'm Matteo Pauroso." }
                p { "I'm a developer exploring software solutions. I enjoy experimenting with new technologies" }
            }
        },
    )
}
