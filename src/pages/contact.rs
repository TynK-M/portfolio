use crate::components::layout;
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "Contact | Matteo Pauroso",
        html! {
            section {
                h1 { "Get in touch" }
                p { "Have a project in mind? Let's talk." }
            }
        },
        "",
    )
}
