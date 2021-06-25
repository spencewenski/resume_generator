use crate::config::Config;
use crate::data::{
    CoverLetter, Education, Objective, OtherExperience, PersonalInfo, ProfessionalExperience,
    ProjectInfo, Resume, Technologies,
};
use crate::renderer::Renderer;
use crate::util::{
    cover_letter_file_name, date_string, split_string_across_lines, time_range_string,
    write_string_to_file, FooterText,
};
use std::path::PathBuf;

pub struct TextRenderer;

impl TextRenderer {
    pub fn new() -> TextRenderer {
        TextRenderer
    }
}

impl Renderer<Resume, PathBuf> for TextRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<PathBuf, String> {
        let ext = String::from("txt");

        if let Some(c) = &element.cover_letter {
            let cover_letter = self.render(c, config)?;
            write_string_to_file(
                &cover_letter,
                config.args.output_dir.as_ref(),
                &cover_letter_file_name(config),
                Some(&ext),
            )?;
        }

        let resume: String = self.render(element, config)?;
        write_string_to_file(
            &resume,
            config.args.output_dir.as_ref(),
            &config.args.output_name,
            Some(&ext),
        )
    }
}

impl Renderer<Resume, String> for TextRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<String, String> {
        let mut text = centered_string(&element.name, config.format_config.text_config.width);
        text = format!(
            "{}\n\n{}",
            text,
            self.render(&element.personal_info, &config)?
        );
        text = format!("{}\n\n{}", text, self.render(&element.objective, &config)?);
        text = format!(
            "{}\n\n{}",
            text,
            self.render(&element.professional_experience, config)?
        );
        if let Some(e) = &element.other_experience {
            text = format!("{}\n\n{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.technologies {
            text = format!("{}\n\n{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.education {
            text = format!("{}\n\n{}", text, self.render(e, config)?);
        }
        let footer = centered_string(
            &FooterText::new().basic_text,
            config.format_config.text_config.width,
        );
        text = format!("{}\n\n{}", text, footer);
        Ok(text)
    }
}

impl Renderer<PersonalInfo, String> for TextRenderer {
    fn render(self: &Self, element: &PersonalInfo, config: &Config) -> Result<String, String> {
        let text = right_and_left_aligned(
            &element.github,
            &element.email,
            config.format_config.text_config.width,
        );
        Ok(text)
    }
}

impl Renderer<Objective, String> for TextRenderer {
    fn render(self: &Self, element: &Objective, config: &Config) -> Result<String, String> {
        let text = split_string_across_lines(
            &element.objective,
            config.format_config.text_config.width,
            None,
            None,
        );
        Ok(text)
    }
}

impl Renderer<Vec<ProfessionalExperience>, String> for TextRenderer {
    fn render(
        self: &Self,
        element: &Vec<ProfessionalExperience>,
        config: &Config,
    ) -> Result<String, String> {
        let mut text = centered_string("EXPERIENCE", config.format_config.text_config.width);

        let exp = element
            .iter()
            .map(|e| self.render(e, config))
            .reduce(|a, b| Ok(format!("{}\n\n{}", a?, b?)))
            .unwrap_or(Err(format!(
                "An error occurred while rendering professional experience to plain text."
            )))?;

        text = format!("{}\n{}", text, exp);

        Ok(text)
    }
}

impl Renderer<ProfessionalExperience, String> for TextRenderer {
    fn render(
        self: &Self,
        element: &ProfessionalExperience,
        config: &Config,
    ) -> Result<String, String> {
        let mut text =
            if let (Some(org), Some(location)) = (&element.organization, &element.location) {
                let text =
                    right_and_left_aligned(org, location, config.format_config.text_config.width);
                format!("{}\n", text)
            } else {
                String::new()
            };
        text = format!(
            "{}{}",
            text,
            right_and_left_aligned(
                &element.position,
                &time_range_string(&element.start, &element.end),
                config.format_config.text_config.width
            )
        );

        let exp = element
            .experience
            .iter()
            .map(|e| {
                split_string_across_lines(
                    e,
                    config.format_config.text_config.width,
                    Some(String::from("- ")),
                    Some(String::from("  ")),
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        text = format!("{}\n{}", text, exp);

        Ok(text)
    }
}

impl Renderer<OtherExperience, String> for TextRenderer {
    fn render(self: &Self, element: &OtherExperience, config: &Config) -> Result<String, String> {
        let header = centered_string("PROJECTS", config.format_config.text_config.width);
        // todo: handle long lines?
        // todo: clean up?
        let projects = element
            .projects
            .iter()
            .map(|s| self.render(s, config))
            .reduce(|a, b| Ok(format!("{}\n{}", a?, b?)))
            .unwrap_or(Err(format!(
                "An error occurred while rendering other experience to plain text."
            )))?;

        Ok(format!("{}\n{}", header, projects))
    }
}

impl Renderer<ProjectInfo, String> for TextRenderer {
    fn render(self: &Self, element: &ProjectInfo, config: &Config) -> Result<String, String> {
        Ok(split_string_across_lines(
            &element.description,
            config.format_config.text_config.width,
            Some(String::from("- ")),
            Some(String::from("  ")),
        ))
    }
}

impl Renderer<Technologies, String> for TextRenderer {
    fn render(self: &Self, element: &Technologies, config: &Config) -> Result<String, String> {
        let header = centered_string("TECHNOLOGIES", config.format_config.text_config.width);
        let technologies = element.technologies.join(", ");
        let technologies = split_string_across_lines(
            &technologies,
            config.format_config.text_config.width,
            None,
            None,
        );
        let technologies = technologies
            .split("\n")
            .map(|x| centered_string(x, config.format_config.text_config.width))
            .collect::<Vec<String>>()
            .join("\n");
        Ok(format!("{}\n{}", header, technologies))
    }
}

impl Renderer<Education, String> for TextRenderer {
    fn render(self: &Self, element: &Education, config: &Config) -> Result<String, String> {
        let mut text = centered_string("UNIVERSITY", config.format_config.text_config.width);
        text = format!(
            "{}\n{}",
            text,
            right_and_left_aligned(
                &element.school,
                &element.location,
                config.format_config.text_config.width
            )
        );
        text = format!(
            "{}\n{}",
            text,
            right_and_left_aligned(
                &element.major,
                &element.graduation,
                config.format_config.text_config.width
            )
        );
        Ok(text)
    }
}

impl Renderer<CoverLetter, String> for TextRenderer {
    fn render(self: &Self, element: &CoverLetter, config: &Config) -> Result<String, String> {
        let mut header = String::new();
        if let Some(name) = &element.name {
            header = format!("{}\n\n", name);
        }
        if let Some(email) = &element.email {
            header = format!("{}{}\n\n", header, email);
        }
        header = format!("{}{}\n\n{}\n\n", header, date_string(), element.salutation);

        let paragraphs = element
            .paragraphs
            .iter()
            .map(|p| {
                split_string_across_lines(p, config.format_config.text_config.width, None, None)
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        let footer = if let Some(name) = &element.name {
            format!("{}\n\n{}", element.closing, name)
        } else {
            String::new()
        };

        Ok(format!("{}{}\n\n{}", header, paragraphs, footer))
    }
}

fn centered_string(s: &str, width: usize) -> String {
    format!("{s:>width$}", s = s, width = (width / 2) + (s.len() / 2))
}

fn right_and_left_aligned(l: &str, r: &str, width: usize) -> String {
    format!(
        "{left}{right:>width$}",
        left = l,
        right = r,
        width = width - l.len()
    )
}

#[cfg(test)]
mod test {
    use crate::config::format_config::{FormatConfig, TextConfig};
    use crate::config::Config;
    use crate::data::{
        CoverLetter, Education, Objective, OtherExperience, PersonalInfo, ProfessionalExperience,
        ProjectInfo, Technologies,
    };
    use crate::renderer::text_renderer::TextRenderer;
    use crate::renderer::Renderer;

    #[test]
    fn test_personal_info() {
        let x = PersonalInfo {
            email: String::from("foo@bar.com"),
            github: String::from("github.com/foo"),
            ..Default::default()
        };
        let rendered = TextRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "github.com/foo                         foo@bar.com"
        );
    }

    #[test]
    fn test_objective() {
        let x = Objective {
            objective: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut"),
        };
        let rendered = TextRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(rendered, "Lorem ipsum dolor sit amet, consectetur adipiscing\nelit, sed do eiusmod tempor incididunt ut");
    }

    #[test]
    fn test_professional_experience() {
        let a = ProfessionalExperience {
            organization: Some(String::from("organizationA")),
            position: String::from("positionA"),
            location: Some(String::from("locationA")),
            start: String::from("startA"),
            end: String::from("endA"),
            experience: vec![
                String::from("experienceA1"),
                String::from("experienceA2"),
                String::from("experienceA3"),
            ],
        };
        let b = ProfessionalExperience {
            organization: Some(String::from("organizationB")),
            position: String::from("positionB"),
            location: Some(String::from("locationB")),
            start: String::from("startB"),
            end: String::from("endB"),
            experience: vec![
                String::from("experienceB1"),
                String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut"),
                String::from("experienceB3"),
            ],
        };
        let x = vec![a, b];

        let rendered = TextRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "                    EXPERIENCE
organizationA                            locationA
positionA                            startA - endA
- experienceA1
- experienceA2
- experienceA3

organizationB                            locationB
positionB                            startB - endB
- experienceB1
- Lorem ipsum dolor sit amet, consectetur
  adipiscing elit, sed do eiusmod tempor
  incididunt ut
- experienceB3"
        );
    }

    #[test]
    fn test_other_experience() {
        let a = ProjectInfo {
            project_name: String::from("project_nameA"),
            description: String::from("descriptionA"),
            url: String::from("example.com"),
            ..Default::default()
        };
        let b = ProjectInfo {
            project_name: String::from("project_nameB"),
            description: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut"),
            url: String::from("example.com"),
            ..Default::default()
        };
        let c = ProjectInfo {
            project_name: String::from("project_nameC"),
            description: String::from("descriptionC"),
            url: String::from("example.com"),
            ..Default::default()
        };
        let x = OtherExperience {
            projects: vec![a, b, c],
        };

        let rendered = TextRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "                     PROJECTS
- descriptionA
- Lorem ipsum dolor sit amet, consectetur
  adipiscing elit, sed do eiusmod tempor
  incididunt ut
- descriptionC"
        );
    }

    #[test]
    fn test_technologies() {
        let tech = vec![
            String::from("Lorem"),
            String::from("ipsum"),
            String::from("dolor"),
            String::from("sit"),
            String::from("amet"),
            String::from("consectetur"),
            String::from("adipiscing"),
            String::from("elit"),
            String::from("sed"),
            String::from("do"),
            String::from("eiusmod"),
            String::from("tempor"),
            String::from("incididunt"),
        ];

        let x = Technologies { technologies: tech };

        let rendered = TextRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "                   TECHNOLOGIES
   Lorem, ipsum, dolor, sit, amet, consectetur,
   adipiscing, elit, sed, do, eiusmod, tempor,
                    incididunt"
        )
    }

    #[test]
    fn test_education() {
        let x = Education {
            school: String::from("school"),
            location: String::from("location"),
            major: String::from("major"),
            graduation: String::from("graduation"),
            ..Default::default()
        };

        let rendered = TextRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "                    UNIVERSITY
school                                    location
major                                   graduation"
        );
    }

    #[test]
    fn test_cover_letter() {
        let x = CoverLetter {
            salutation: String::from("Hello,"),
            closing: String::from("From,"),
            paragraphs: vec!["foo", "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut", "baz"]
                .into_iter()
                .map(|x| String::from(x))
                .collect(),
            name: Some(String::from("Foo Bar")),
            email: Some(String::from("foo@bar.com")),
        };

        let rendered = TextRenderer::new().render(&x, &get_config()).unwrap();
        assert_eq!(rendered, "Foo Bar\n\nfoo@bar.com\n\n24 June 2021\n\nHello,\n\nfoo\n\nLorem ipsum dolor sit amet, consectetur adipiscing\nelit, sed do eiusmod tempor incididunt ut\n\nbaz\n\nFrom,\n\nFoo Bar");
    }

    fn get_config() -> Config {
        Config {
            format_config: FormatConfig {
                text_config: TextConfig { width: 50 },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}
