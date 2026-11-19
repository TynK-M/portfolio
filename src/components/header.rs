use maud::{Markup, html};

pub fn render() -> Markup {
    html! {
        header class="site-header" {
            a class="site-logo" href="/" {
                "Matteo Pauroso"
            }

            nav aria-label="Main navigation" {
                ul {
                    li {
                        a href="/about/" { "About" }
                    }
                    li {
                        a href="/projects/" { "Projects" }
                    }
                    li {
                        a href="/contact/" { "Contact" }
                    }
                }
            }
        }
    }
}
