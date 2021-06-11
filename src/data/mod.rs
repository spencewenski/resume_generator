use crate::data::parser::{TomlParser, Parser};

mod parser;

impl Resume {
    pub fn read_from_config_file(file_name: &str) -> Result<Resume, String> {
        TomlParser::parse(file_name)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Resume {
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
    pub name: String,
    pub email: String,
    pub phone: String,
    pub github: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<String>>,
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
    pub projects: Vec<String>,
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
            name: String::from("Foo Bar"),
            email: String::from("foo@example.com"),
            phone: String::from("1-555-555-5555"),
            github: String::from("github.com/foo"),
            other: Option::Some(vec![ String::from("Foo"), String::from("Bar") ]),
        };

        assert_eq!(personal_info.name, String::from("Foo Bar"));
        assert_eq!(personal_info.email, String::from("foo@example.com"));
        assert_eq!(personal_info.phone, String::from("1-555-555-5555"));
        assert_eq!(personal_info.github, String::from("github.com/foo"));
        assert_eq!(personal_info.other, Option::Some(vec![String::from("Foo"), String::from("Bar")]));
    }
}
