mod parser;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resume {
    pub personal_info: PersonalInfo,
    pub objective: Objective,
    pub professional_experience: Vec<ProfessionalExperience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub education: Option<Vec<Education>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_experience: Option<OtherExperience>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Objective {
    pub objective: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfessionalExperience {
    pub organization: String,
    pub location: String,
    pub position: String,
    pub start: String,
    pub end: String,
    pub experience: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub school: String,
    pub location: String,
    pub start: String,
    pub end: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extras: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OtherExperience {
    pub experience: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::data::PersonalInfo;

    #[test]
    fn test_personal_info() {
        let personal_info = PersonalInfo {
            name: String::from("Foo Bar"),
            email: Option::Some(String::from("foo@example.com")),
            phone: Option::Some(String::from("1-555-555-5555")),
            website: Option::Some(String::from("www.example.com")),
            other: Option::Some(vec![ String::from("Foo"), String::from("Bar") ]),
        };

        assert_eq!(personal_info.name, String::from("Foo Bar"));
        assert_eq!(personal_info.email, Option::Some(String::from("foo@example.com")));
        assert_eq!(personal_info.phone, Option::Some(String::from("1-555-555-5555")));
        assert_eq!(personal_info.website, Option::Some(String::from("www.example.com")));
        assert_eq!(personal_info.other, Option::Some(vec![String::from("Foo"), String::from("Bar")]));
    }
}
