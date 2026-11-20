use crate::components::{education_item, experience_item, layout};
use maud::{Markup, html};

#[derive(Clone, Copy)]
enum EmploymentType {
    FullTime,
    Stage,
}

impl EmploymentType {
    fn label(self) -> &'static str {
        match self {
            Self::FullTime => "Full-time",
            Self::Stage => "Stage",
        }
    }
}

struct Experience {
    role: &'static str,
    company: &'static str,
    employment_type: EmploymentType,
    period: &'static str,
    description: &'static str,
}

struct Education {
    degree: &'static str,
    institution: &'static str,
    period: &'static str,
    grade: Option<&'static str>,
    description: &'static str,
}

const EXPERIENCES: &[Experience] = &[
    Experience {
        role: "Software Developer",
        company: "Poste Italiane",
        employment_type: EmploymentType::FullTime,
        period: "Oct 2026 - Current",
        description: "Function Digital, Technology & Operations.",
    },
    Experience {
        role: "Backend Developer",
        company: "Octo Telematics",
        employment_type: EmploymentType::FullTime,
        period: "Jan 2026 - Apr 2026",
        description: "Experience in the Integration team, developing backend solutions in Java, Python, and TypeScript for the maintenance and evolution of internal applications.

        Design and implementation of distributed services, integration between enterprise systems, and optimization of operational workflows.",
    },
    Experience {
        role: "Backend Developer",
        company: "ELIS Innovation Hub",
        employment_type: EmploymentType::Stage,
        period: "Jun 2025 - Dec 2025",
        description: "External collaborator at Octo Telematics.

        Backend development in Java and Python for the maintenance and evolution of internal applications. Involved in the design and implementation of distributed services, focusing on system integration, optimization of business workflows, and replacing external solutions with more efficient in-house applications.",
    },
];

const EDUCATIONS: &[Education] = &[
    Education {
        degree: "Computer Science Degree",
        institution: "UnitelmaSapienza",
        period: "Currently studying",
        grade: None,
        description: "Currently studying to increse my Computer Science knownledge.",
    },
    Education {
        degree: "Bootcamp Junior Developer Java Python",
        institution: "ELIS Innovation Hub",
        period: "May 2025 - Jun 2025",
        grade: None,
        description: "An intensive six-week bootcamp organized by ELIS Innovation Hub, focused on software development and technologies such as Java (with particular emphasis on Spring Boot and Spring Cloud), Python, Docker, relational and non-relational databases.

        The program also included training in tools such as Excel, PowerPoint, and Word.

        The goal was to develop both technical and practical skills through hands-on exercises, collaborative activities, and simulations of real-world enterprise environments.",
    },
    Education {
        degree: "Junior Java Developer Course",
        institution: "Generation Italy",
        period: "Dec 2024 - Apr 2025",
        grade: None,
        description: "An intensive 500-hour training course covering the fundamentals of object-oriented programming and Java, as well as key frontend technologies (HTML, CSS, and JavaScript) and relational databases (MySQL).

        The program also included a soft skills component, covering problem-solving, teamwork, communication, and Agile methodologies, including SCRUM.",
    },
    Education {
        degree: "Physics Degree",
        institution: "Sapienza",
        period: "Interrupted studies",
        grade: None,
        description: "I successfully completed the Computational Laboratory and Computational Physics Laboratory courses, gaining hands-on programming experience by developing simulations of physical environments in C and analyzing and visualizing data using Python.

        I interrupted my studies for personal reasons, but retained a solid foundation in computational skills that has supported my career path in the technology sector.",
    },
    Education {
        degree: "High School Diploma in Mechatronics",
        institution: "I.I.S. Giorgi - Woolf",
        period: "Sep 2017 - Jul 2022",
        grade: Some("100/100"),
        description: "",
    },
];

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

                    @for education in EDUCATIONS {
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
        &crate::styles::about::stylesheet(),
    )
}
