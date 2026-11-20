use maud::{Markup, html};

pub fn render() -> Markup {
    html! {
        header class="site-header" {
            a class="site-logo" href=(crate::site::url("/")) {
                img
                    src=(crate::site::url("logo.png"))
                    alt=""
                    width="32"
                    height="32";
                span { "Matteo Pauroso" }
            }

            nav aria-label="Main navigation" {
                ul {
                    li {
                        a href=(crate::site::url("about/")) { "About" }
                    }
                    li {
                        a href=(crate::site::url("projects/")) { "Projects" }
                    }
                    li {
                        a href=(crate::site::url("contact/")) { "Contact" }
                    }
                }
            }
        }
    }
}
