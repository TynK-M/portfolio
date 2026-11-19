use maud::{Markup, html};

pub fn render() -> Markup {
    html! {
        footer {
            p { "© 2026 Matteo Pauroso" }
        }
    }
}
