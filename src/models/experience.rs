#[derive(Clone, Copy)]
pub enum EmploymentType {
    FullTime,
    Stage,
}

impl EmploymentType {
    pub fn label(self) -> &'static str {
        match self {
            Self::FullTime => "Full-time",
            Self::Stage => "Stage",
        }
    }
}

pub struct Experience {
    pub role: &'static str,
    pub company: &'static str,
    pub employment_type: EmploymentType,
    pub period: &'static str,
    pub description: &'static str,
}
