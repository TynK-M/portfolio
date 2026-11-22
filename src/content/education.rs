use crate::models::education::Education;

pub const EDUCATION: &[Education] = &[
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
