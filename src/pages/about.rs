use crate::{
    components::{education_item, experience_item, layout},
    content::{education::EDUCATION, experiences::EXPERIENCES},
};
use maud::{Markup, html};

pub fn render() -> Markup {
    layout::render(
        "About | Matteo Pauroso",
        html! {
            section {
                h1 { "About me" }

                p {
                    "I'm a Software Developer with experience in developing and maintaining server-side applications and systems."
                }

                p {
                    "Throughout my career, I've primarily worked with Go, Java, and Python, using them to develop APIs, manage data, and build backend solutions. I see programming languages as tools that serve the project: technology choices should be driven by requirements, objectives, and context rather than personal preference."
                }

                p {
                    "I've gained experience with relational databases, particularly PostgreSQL, as well as technologies for data and message exchange, such as Apache Kafka. I also work with tools and technologies including Git, Docker, and Kubernetes."
                }

                p {
                    "My development approach emphasizes code quality, thoughtful design, and the delivery of reliable, maintainable, and scalable solutions. I tackle problems pragmatically, evaluating available alternatives and choosing the technologies best suited to each situation."
                }

                p {
                    "I consider communication and collaboration within a team essential to both the development process and professional growth. I'm continuously exploring new technologies and methodologies, driven by curiosity and an interest in the ever-evolving software landscape."
                }

                section class="about-experience" {
                    h2 { "Work Experience" }

                    @for experience in EXPERIENCES {
                        (experience_item::render(
                            experience.role,
                            experience.company,
                            experience.employment_type.label(),
                            experience.period,
                            experience.description,
                        ))
                    }
                }

                section class="about-education" {
                    h2 { "Education" }

                    @for education in EDUCATION {
                        (education_item::render(
                            education.degree,
                            education.institution,
                            education.period,
                            education.grade,
                            education.description,
                        ))
                    }
                }
            }
        },
        &crate::styles::pages::about::stylesheet(),
    )
}
