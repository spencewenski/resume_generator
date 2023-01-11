use crate::config::Config;
use crate::data::{
    Education, Objective, OtherExperience, OtherPersonalInfo, PersonalInfo, ProfessionalExperience,
    ProjectInfo, Resume, Technologies,
};
use crate::renderer::Renderer;
use crate::util::{
    add_https_to_url, split_string_across_lines, time_range_string, write_string_to_file,
    FooterText,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn new() -> MarkdownRenderer {
        MarkdownRenderer::default()
    }
}

impl Renderer<Resume, PathBuf> for MarkdownRenderer {
    fn render(&self, element: &Resume, config: &Config) -> Result<PathBuf, String> {
        let s: String = self.render(element, config)?;

        write_string_to_file(
            &s,
            config.args.output_dir.as_ref(),
            &config.args.output_name,
            Some(String::from("md")).as_ref(),
        )
    }
}

impl Renderer<Resume, String> for MarkdownRenderer {
    fn render(&self, element: &Resume, config: &Config) -> Result<String, String> {
        let mut text = format!("# {}", element.name);
        if let Some(obj) = &element.objective {
            text = format!("{}\n\n{}", text, self.render(obj, config)?);
        }
        text = format!(
            "{}\n\n{}",
            text,
            self.render(&element.personal_info, config)?
        );
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

        let footer_text = FooterText::new();
        text = format!(
            "{}\n\n---\n\n{} [{}]({})\n",
            text,
            footer_text.prefix,
            footer_text.url,
            add_https_to_url(&footer_text.url)
        );

        Ok(text)
    }
}

impl Renderer<PersonalInfo, String> for MarkdownRenderer {
    fn render(&self, element: &PersonalInfo, config: &Config) -> Result<String, String> {
        let mut text = "## Find me here".to_string();
        text = format!(
            "{}\n- Email: [{}](mailto:{})",
            text, element.email, element.email
        );
        text = format!(
            "{}\n- GitHub: [{}]({})",
            text,
            element.github,
            add_https_to_url(&element.github)
        );

        let other_info = element
            .other
            .as_ref()
            .map(|other| -> Result<String, String> {
                let info = other
                    .iter()
                    .map(|e| self.render(e, config))
                    .reduce(|a, b| Ok(format!("{}\n{}", a?, b?)))
                    .unwrap_or_else(|| {
                        Err(String::from(
                            "An error occurred while rendering personal info to markdown",
                        ))
                    });
                info
            });

        if let Some(other_info) = other_info {
            text = format!("{}\n{}", text, other_info?);
        }

        Ok(text)
    }
}

impl Renderer<OtherPersonalInfo, String> for MarkdownRenderer {
    fn render(&self, element: &OtherPersonalInfo, _config: &Config) -> Result<String, String> {
        if let Some(url) = &element.url {
            Ok(format!(
                "- {}: [{}]({})",
                element.item,
                url,
                add_https_to_url(url)
            ))
        } else {
            Ok(format!("- {}", element.item))
        }
    }
}

impl Renderer<Objective, String> for MarkdownRenderer {
    fn render(&self, element: &Objective, config: &Config) -> Result<String, String> {
        Ok(split_string_across_lines(
            &element.objective,
            config.format_config.markdown_config.width,
            None,
            None,
        ))
    }
}

impl Renderer<Vec<ProfessionalExperience>, String> for MarkdownRenderer {
    fn render(
        &self,
        element: &Vec<ProfessionalExperience>,
        config: &Config,
    ) -> Result<String, String> {
        let mut text = "## Experience".to_string();

        let exp = element
            .iter()
            .map(|e| self.render(e, config))
            .reduce(|a, b| Ok(format!("{}\n\n{}", a?, b?)))
            .unwrap_or_else(|| {
                Err(String::from(
                    "An error occurred while rendering professional experience to markdown.",
                ))
            })?;

        text = format!("{}\n{}", text, exp);

        Ok(text)
    }
}

impl Renderer<ProfessionalExperience, String> for MarkdownRenderer {
    fn render(&self, element: &ProfessionalExperience, config: &Config) -> Result<String, String> {
        let mut text =
            if let (Some(org), Some(location)) = (&element.organization, &element.location) {
                format!("### {} - {}", org, location)
            } else {
                String::new()
            };
        text = format!(
            "{}\n```\n{}\n{}\n```",
            text,
            element.position,
            time_range_string(&element.start, &element.end)
        );

        let exp = element
            .experience
            .iter()
            .map(|e| {
                Ok(split_string_across_lines(
                    e,
                    config.format_config.markdown_config.width,
                    Some(String::from("- ")),
                    Some(String::from("  ")),
                ))
            })
            .reduce(|a, b| Ok(format!("{}\n{}", a?, b?)))
            .unwrap_or_else(|| {
                Err(String::from(
                    "An error occurred while rendering professional experience to markdown.",
                ))
            })?;

        text = format!("{}\n{}", text, exp);

        Ok(text)
    }
}

impl Renderer<OtherExperience, String> for MarkdownRenderer {
    fn render(&self, element: &OtherExperience, config: &Config) -> Result<String, String> {
        let mut text = "## Projects".to_string();

        // todo: clean up?
        let projects = element
            .get_projects_for_resume()
            .into_iter()
            .map(|e| self.render(e, config))
            .reduce(|a, b| Ok(format!("{}\n{}", a?, b?)))
            .unwrap_or_else(|| {
                Err("An error occurred while rendering other experience to markdown.".to_string())
            })?;

        text = format!("{}\n{}", text, projects);

        Ok(text)
    }
}

impl Renderer<ProjectInfo, String> for MarkdownRenderer {
    fn render(&self, element: &ProjectInfo, config: &Config) -> Result<String, String> {
        let project_info = format!(
            "[{}]({}) - {}",
            element.project_name,
            add_https_to_url(&element.url),
            element.description
        );
        Ok(split_string_across_lines(
            &project_info,
            config.format_config.markdown_config.width,
            Some(String::from("- ")),
            Some(String::from("  ")),
        ))
    }
}

impl Renderer<Technologies, String> for MarkdownRenderer {
    fn render(&self, element: &Technologies, _config: &Config) -> Result<String, String> {
        Ok(format!(
            "## Technologies\n{}",
            element.technologies.join(", ")
        ))
    }
}

impl Renderer<Education, String> for MarkdownRenderer {
    fn render(&self, element: &Education, _config: &Config) -> Result<String, String> {
        let mut text = format!("## University\n### {}", element.school);
        if element.graduation.is_some() {
            text = format!(
                "{}\n```\n{}\n{}\n{}\n```",
                text,
                element.major,
                element.graduation.as_ref().unwrap(),
                element.location
            );
        } else {
            text = format!(
                "{}\n```\n{}\n{}\n```",
                text, element.major, element.location
            );
        }
        Ok(text)
    }
}

#[cfg(test)]
mod test {
    use crate::config::format_config::{FormatConfig, TextConfig};
    use crate::config::Config;
    use crate::data::{
        Education, Objective, OtherExperience, OtherPersonalInfo, PersonalInfo,
        ProfessionalExperience, ProjectInfo, Technologies,
    };
    use crate::renderer::markdown_renderer::MarkdownRenderer;
    use crate::renderer::Renderer;

    #[test]
    fn test_personal_info() {
        let a = OtherPersonalInfo {
            item: String::from("Foo"),
            url: Some(String::from("example.com/foo")),
        };
        let b = OtherPersonalInfo {
            item: String::from("Bar"),
            url: Some(String::from("example.com/bar")),
        };
        let x = PersonalInfo {
            email: String::from("foo@bar.com"),
            github: String::from("github.com/foo"),
            other: Some(vec![a, b]),
        };
        let rendered = MarkdownRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "## Find me here\n- Email: [foo@bar.com](mailto:foo@bar.com)\n- GitHub: [github.com/foo](https://github.com/foo)\n- Foo: [example.com/foo](https://example.com/foo)\n- Bar: [example.com/bar](https://example.com/bar)"
        );
    }

    #[test]
    fn test_objective() {
        let x = Objective {
            objective: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut"),
        };
        let rendered = MarkdownRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(rendered, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut");
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

        let rendered = MarkdownRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "## Experience\n### organizationA - locationA\n```\npositionA\nstartA - endA\n```\n- experienceA1\n- experienceA2\n- experienceA3\n\n### organizationB - locationB\n```\npositionB\nstartB - endB\n```\n- experienceB1\n- Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut\n- experienceB3"
        );
    }

    #[test]
    fn test_other_experience() {
        let a = ProjectInfo {
            project_name: String::from("project_nameA"),
            description: String::from("descriptionA"),
            url: String::from("example.com"),
            include_on_resume: true,
            ..Default::default()
        };
        let b = ProjectInfo {
            project_name: String::from("project_nameB"),
            description: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut"),
            url: String::from("example.com"),
            include_on_resume: true,
            ..Default::default()
        };
        let c = ProjectInfo {
            project_name: String::from("project_nameC"),
            description: String::from("descriptionC"),
            url: String::from("example.com"),
            include_on_resume: true,
            ..Default::default()
        };
        let x = OtherExperience {
            projects: vec![a, b, c],
        };

        let rendered = MarkdownRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "## Projects\n- [project_nameA](https://example.com) - descriptionA\n- [project_nameB](https://example.com) - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor\n  incididunt ut\n- [project_nameC](https://example.com) - descriptionC"
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

        let rendered = MarkdownRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "## Technologies\nLorem, ipsum, dolor, sit, amet, consectetur, adipiscing, elit, sed, do, eiusmod, tempor, incididunt"
        )
    }

    #[test]
    fn test_education() {
        let x = Education {
            school: String::from("school"),
            location: String::from("location"),
            major: String::from("major"),
            graduation: Some(String::from("graduation")),
            ..Default::default()
        };

        let rendered = MarkdownRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "## University\n### school\n```\nmajor\ngraduation\nlocation\n```"
        );
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
