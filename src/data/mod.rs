use crate::config::Config;
use crate::util::{default_true, string_from_file, toml_from_string};
use std::collections::HashSet;

impl Resume {
    pub fn read_from_config_file(file_name: &str, config: &Config) -> Result<Resume, String> {
        let s = string_from_file(file_name)?;
        // todo: support other config file formats?
        let mut resume: Resume = toml_from_string(&s)?;
        if let Some(email) = &config.args.email {
            resume.personal_info.email = email.to_owned()
        }
        if let Some(cover_letter) = &mut resume.cover_letter {
            cover_letter.name = Some(resume.name.clone());
            cover_letter.email = Some(resume.personal_info.email.clone());
        }
        Resume::verify(resume)
    }

    fn verify(resume: Resume) -> Result<Resume, String> {
        if let Some(tech) = &resume.technologies {
            Resume::verify_technologies(tech)?;
        }
        Ok(resume)
    }

    fn verify_technologies(tech: &Technologies) -> Result<(), String> {
        let mut tech_set = HashSet::new();
        for t in tech.technologies.iter() {
            if tech_set.contains(t) {
                return Err(format!(
                    "Technologies list contains a duplicate entry: {}",
                    t
                ));
            } else {
                tech_set.insert(t);
            }
        }
        Ok(())
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_letter: Option<CoverLetter>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PersonalInfo {
    // todo: use the first to elements of a vector as the left/right items in the header
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
    /// This is allowed to be omitted so that experience with the same company
    /// but at different levels can be combined under one company header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// This is allowed to be omitted so that experience with the same company
    /// but at different levels can be combined under one company header.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
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

impl OtherExperience {
    pub fn get_projects(&self) -> Vec<&ProjectInfo> {
        self.projects.iter().collect()
    }

    pub fn get_projects_for_resume(&self) -> Vec<&ProjectInfo> {
        self.projects
            .iter()
            .filter(|x| x.include_on_resume)
            .collect()
    }

    pub fn get_projects_for_github(&self) -> Vec<&ProjectInfo> {
        self.projects
            .iter()
            .filter(|x| x.include_on_github)
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ProjectInfo {
    pub project_name: String,
    pub url: String,
    pub description: String,
    #[serde(default = "default_true")]
    pub include_on_resume: bool,
    #[serde(default = "default_true")]
    pub include_on_github: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Technologies {
    pub technologies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CoverLetter {
    pub paragraphs: Vec<String>,
    /// Should be populated by impl Resume
    #[serde(skip_serializing, skip_deserializing)]
    pub name: Option<String>,
    /// Should be populated by impl Resume
    #[serde(skip_serializing, skip_deserializing)]
    pub email: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::data::{Resume, Technologies};

    #[test]
    fn test_deserialize_toml() {
        let resume =
            Resume::read_from_config_file("tst/test_resume.toml", &Default::default()).unwrap();

        assert_eq!(resume.name, "Foo Bar");

        assert_eq!(resume.personal_info.email, String::from("foo@example.com"));
        assert_eq!(resume.personal_info.github, String::from("github.com/foo"));

        assert!(resume.personal_info.other.is_some());
        let personal_info = resume.personal_info.other.unwrap();
        assert_eq!(personal_info.len(), 2);
        assert_eq!(personal_info[0].item, String::from("Foo"));
        assert_eq!(
            personal_info[0].url,
            Some(String::from("https://example.com"))
        );
        assert_eq!(personal_info[1].item, String::from("Bar"));
        assert!(personal_info[1].url.is_none());

        assert_eq!(resume.objective.objective, "objective");

        assert_eq!(resume.professional_experience.len(), 2);
        assert_eq!(
            resume.professional_experience[0].organization,
            Some(String::from("organizationA"))
        );
        assert_eq!(
            resume.professional_experience[0].location,
            Some(String::from("locationA"))
        );
        assert_eq!(
            resume.professional_experience[0].position,
            String::from("positionA")
        );
        assert_eq!(
            resume.professional_experience[0].start,
            String::from("startA")
        );
        assert_eq!(resume.professional_experience[0].end, String::from("endA"));
        assert_eq!(
            resume.professional_experience[0].experience,
            vec![
                String::from("FooA"),
                String::from("BarA"),
                String::from("BazA")
            ]
        );

        assert_eq!(
            resume.professional_experience[1].organization,
            Some(String::from("organizationB"))
        );
        assert_eq!(
            resume.professional_experience[1].location,
            Some(String::from("locationB"))
        );
        assert_eq!(
            resume.professional_experience[1].position,
            String::from("positionB")
        );
        assert_eq!(
            resume.professional_experience[1].start,
            String::from("startB")
        );
        assert_eq!(resume.professional_experience[1].end, String::from("endB"));
        assert_eq!(
            resume.professional_experience[1].experience,
            vec![
                String::from("FooB"),
                String::from("BarB"),
                String::from("BazB")
            ]
        );

        assert!(resume.education.is_some());
        let education = resume.education.unwrap();
        assert_eq!(education.school, String::from("school"));
        assert_eq!(education.location, String::from("location"));
        assert_eq!(education.major, String::from("major"));
        assert_eq!(education.graduation, String::from("graduation"));
        assert_eq!(
            education.extras,
            Some(vec![
                String::from("Foo"),
                String::from("Bar"),
                String::from("Baz")
            ])
        );

        assert!(resume.other_experience.is_some());
        let exp = resume.other_experience.unwrap();
        assert_eq!(exp.projects.len(), 1);
        assert_eq!(exp.projects[0].project_name, String::from("Project Name"));
        assert_eq!(exp.projects[0].url, String::from("https://example.com"));
        assert_eq!(exp.projects[0].description, String::from("Description"));

        assert!(resume.cover_letter.is_some());
        let cover_letter = resume.cover_letter.unwrap();
        assert_eq!(cover_letter.paragraphs.len(), 3);
        assert_eq!(cover_letter.paragraphs[0], "foo");
        assert_eq!(cover_letter.paragraphs[1], "bar");
        assert_eq!(cover_letter.paragraphs[2], "baz");
        assert_eq!(cover_letter.name.unwrap(), resume.name);
        assert_eq!(cover_letter.email.unwrap(), resume.personal_info.email);
    }

    #[test]
    fn test_verify_technologies() {
        let t = Technologies {
            technologies: vec!["foo", "bar", "baz", "foo", "things", "stuff"]
                .into_iter()
                .map(|x| String::from(x))
                .collect(),
        };
        let r = Resume::verify_technologies(&t);
        assert!(r.is_err());
    }
}
