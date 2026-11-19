use crate::components::layout;
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "About | Matteo Pauroso",
        html! {
            section {
                h1 { "About me" }
                p { "A little introduction about myself." }
            }
        },
    )
}
