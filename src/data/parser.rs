use std::fs;
use crate::data::Resume;

pub trait Parser {
    fn parse(file_name: &str) -> Result<Resume, String>;
}

pub struct TomlParser;

impl Parser for TomlParser {
    fn parse(file_name: &str) -> Result<Resume, String> {
        let input = fs::read_to_string(file_name).map_err(|e| {
            format!("An error occurred while trying to open file [{}]: {}", file_name, e)
        })?;

        toml::from_str::<Resume>(&*input).map_err(|e| {
            format!("An error occurred while parsing resume toml: {}", e)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::data::parser::{Parser, TomlParser};

    #[test]
    fn test_deserialize_toml() {
        let resume = TomlParser::parse("tst/test_resume.toml").unwrap();

        assert_eq!(resume.personal_info.name, "Foo Bar");
        assert_eq!(resume.personal_info.email, Option::Some(String::from("foo@example.com")));
        assert_eq!(resume.personal_info.phone, Option::Some(String::from("1-555-555-5555")));
        assert_eq!(resume.personal_info.website, Option::Some(String::from("example.com")));
        assert_eq!(resume.personal_info.other, Option::Some(vec![String::from("github.com/example"),
                                                                 String::from("gitlab.com/example"),
                                                                 String::from("linkedin.com/example")]));

        assert_eq!(resume.objective.objective, "objective");

        assert_eq!(resume.professional_experience.len(), 2);
        assert_eq!(resume.professional_experience[0].organization, String::from("organizationA"));
        assert_eq!(resume.professional_experience[0].location, String::from("locationA"));
        assert_eq!(resume.professional_experience[0].position, String::from("positionA"));
        assert_eq!(resume.professional_experience[0].start, String::from("startA"));
        assert_eq!(resume.professional_experience[0].end, String::from("endA"));
        assert_eq!(resume.professional_experience[0].experience, vec![String::from("FooA"),
                                                                 String::from("BarA"),
                                                                 String::from("BazA")]);

        assert_eq!(resume.professional_experience[1].organization, String::from("organizationB"));
        assert_eq!(resume.professional_experience[1].location, String::from("locationB"));
        assert_eq!(resume.professional_experience[1].position, String::from("positionB"));
        assert_eq!(resume.professional_experience[1].start, String::from("startB"));
        assert_eq!(resume.professional_experience[1].end, String::from("endB"));
        assert_eq!(resume.professional_experience[1].experience, vec![String::from("FooB"),
                                                                 String::from("BarB"),
                                                                 String::from("BazB")]);

        assert!(resume.education.is_some());
        let education = resume.education.unwrap();
        assert_eq!(education.len(), 1);
        assert_eq!(education[0].school, String::from("school"));
        assert_eq!(education[0].location, String::from("location"));
        assert_eq!(education[0].start, String::from("start"));
        assert_eq!(education[0].end, String::from("end"));
        assert_eq!(education[0].extras, Option::Some(vec![String::from("Foo"),
                                                          String::from("Bar"),
                                                          String::from("Baz")]));

        assert!(resume.other_experience.is_some());
        let other_experience = resume.other_experience.unwrap();
        assert_eq!(other_experience.experience, vec![String::from("Foo"),
                                                          String::from("Bar"),
                                                          String::from("Baz")]);
    }
}
