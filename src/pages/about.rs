use crate::components::layout;
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "About | Matteo Pauroso",
        html! {
            section {
                h1 { "About me" }
                p { "I'm a Backend Developer with experience in developing and maintaining server-side applications and APIs." }

                p { "I mainly work with Java and Python. I'm interested in building simple, maintainable and reliable systems while continuously improving through hands-on experience and personal projects." }

                p{"Currently studying Computer Science at UnitelmaSapienza." }
            }
        },
        "",
    )
}
