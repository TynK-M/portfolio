use crate::components::layout;
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "Contact | Matteo Pauroso",
        html! {
            section class="contact" {
                h1 { "Get in touch" }

                p { "Have a project in mind? Let's talk." }

                ul class="contact-links" {
                    li {
                        a
                            href="https://github.com/TynK-M"
                            target="_blank"
                            rel="noopener noreferrer" {
                                img
                                    src="https://cdn.simpleicons.org/github/ebdbb2"
                                    alt=""
                                    width="24"
                                    height="24";
                                span { "GitHub" }
                            }
                    }

                    li {
                        a
                            href="https://www.linkedin.com/in/matteo-pauroso/"
                            target="_blank"
                            rel="noopener noreferrer" {
                                img
                                    src="https://cdn.jsdelivr.net/gh/devicons/devicon@latest/icons/linkedin/linkedin-original.svg"
                                    alt=""
                                    width="24"
                                    height="24";
                                span { "LinkedIn" }
                            }
                    }
                }
            }
        },
        &crate::styles::pages::contact::stylesheet(),
    )
}
