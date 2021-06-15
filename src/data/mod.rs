use crate::data::parser::{Parser, TomlParser};

mod parser;

impl Resume {
    pub fn read_from_config_file(file_name: &str) -> Result<Resume, String> {
        TomlParser::parse(file_name)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Resume {
    pub name: String,
    pub personal_info: PersonalInfo,
    pub objective: Objective,
    pub professional_experience: Vec<ProfessionalExperience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<Education>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_experience: Option<OtherExperience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub technologies: Option<Technologies>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersonalInfo {
    pub email: String,
    pub github: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<OtherPersonalInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OtherPersonalInfo {
    pub item: String,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Objective {
    pub objective: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProfessionalExperience {
    pub organization: String,
    pub location: String,
    pub position: String,
    pub start: String,
    pub end: String,
    pub experience: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Education {
    pub school: String,
    pub location: String,
    pub major: String,
    pub graduation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OtherExperience {
    pub projects: Vec<ProjectInfo>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectInfo {
    pub project_name: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Technologies {
    pub technologies: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::data::PersonalInfo;

    #[test]
    fn test_personal_info() {
        let personal_info = PersonalInfo {
            email: String::from("foo@example.com"),
            github: String::from("github.com/foo"),
            ..Default::default()
        };

        assert_eq!(personal_info.email, String::from("foo@example.com"));
    }
}
