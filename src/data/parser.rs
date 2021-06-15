use crate::data::Resume;
use crate::util::{string_from_file, toml_from_string};

pub trait Parser {
    fn parse(file_name: &str) -> Result<Resume, String>;
}

pub struct TomlParser;

impl Parser for TomlParser {
    fn parse(file_name: &str) -> Result<Resume, String> {
        let input = string_from_file(file_name)?;
        toml_from_string(&input)
    }
}

#[cfg(test)]
mod test {
    use crate::data::parser::{Parser, TomlParser};

    #[test]
    fn test_deserialize_toml() {
        let resume = TomlParser::parse("tst/test_resume.toml").unwrap();

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
            String::from("organizationA")
        );
        assert_eq!(
            resume.professional_experience[0].location,
            String::from("locationA")
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
            String::from("organizationB")
        );
        assert_eq!(
            resume.professional_experience[1].location,
            String::from("locationB")
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
    }
}
