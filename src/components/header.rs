use maud::{Markup, html};

pub fn render() -> Markup {
    html! {
        header class="site-header" {
            a class="site-logo" href=(crate::config::url("/")) {
                img
                    src=(crate::config::url("logo.png"))
                    alt=""
                    width="32"
                    height="32";
                span { "Matteo Pauroso" }
            }

            nav aria-label="Main navigation" {
                ul {
                    li {
                        a href=(crate::config::url("about/")) { "About" }
                    }
                    li {
                        a href=(crate::config::url("projects/")) { "Projects" }
                    }
                    li {
                        a href=(crate::config::url("contact/")) { "Contact" }
                    }
                }
            }
        }
    }
}
