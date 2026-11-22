use crate::models::experience::{EmploymentType, Experience};

pub const EXPERIENCES: &[Experience] = &[
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
        description: "Experience in the Integration team, developing backend solutions in Java, Python, and TypeScript for the maintenance and evolution of internal applications.

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
