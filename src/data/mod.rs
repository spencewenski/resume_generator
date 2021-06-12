use crate::data::parser::{TomlParser, Parser};

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
    // todo: put everything else in OtherPersonalInfo?
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub github: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<OtherPersonalInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OtherPersonalInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_description: Option<String>,
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
            phone: Some(String::from("1-555-555-5555")),
            github: String::from("github.com/foo"),
            other: Some(vec![ String::from("Foo"), String::from("Bar") ]),
        };

        assert_eq!(personal_info.name, String::from("Foo Bar"));
        assert_eq!(personal_info.email, String::from("foo@example.com"));
        assert_eq!(personal_info.phone, Some(String::from("1-555-555-5555")));
        assert_eq!(personal_info.github, String::from("github.com/foo"));
        assert_eq!(personal_info.other, Some(vec![String::from("Foo"), String::from("Bar")]));
    }
}
