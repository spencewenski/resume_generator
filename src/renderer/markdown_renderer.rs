use crate::renderer::Renderer;
use crate::data::{Resume, PersonalInfo, Objective, ProfessionalExperience, OtherExperience, Technologies, Education, ProjectInfo, OtherPersonalInfo};
use std::path::PathBuf;
use crate::config::Config;
use crate::util::{write_string_to_file, add_https_to_url, time_range_string, split_string_across_lines, FooterText};

pub struct MarkdownRenderer;

impl MarkdownRenderer {
    pub fn new() -> MarkdownRenderer {
        MarkdownRenderer
    }
}

impl Renderer<Resume, PathBuf> for MarkdownRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<PathBuf, String> {
        let s: String = self.render(element, config)?;

        write_string_to_file(&s,
                             config.args.output_dir.as_ref(),
                             &config.args.output_name,
                             Some(String::from("md")).as_ref())
    }
}

impl Renderer<Resume, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<String, String> {
        let mut text = format!("# {}", element.name);
        text = format!("{}\n\n{}", text, self.render(&element.objective, config)?);
        text = format!("{}\n\n{}", text, self.render(&element.personal_info, config)?);
        text = format!("{}\n\n{}", text, self.render(&element.professional_experience, config)?);
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
        text = format!("{}\n\n---\n\n{} [{}]({})\n",
                       text,
                       footer_text.prefix,
                       footer_text.url,
                       add_https_to_url(&footer_text.url));

        Ok(text)
    }
}

impl Renderer<PersonalInfo, String> for MarkdownRenderer {
    fn render(self: &Self, element: &PersonalInfo, config: &Config) -> Result<String, String> {
        let mut text = format!("## Links and Contact Info");
        text = format!("{}\n- Email: [{}](mailto:{})", text, element.email, element.email);
        text = format!("{}\n- GitHub: [{}]({})", text, element.github, add_https_to_url(&element.github));

        let other_info = element.other.as_ref()
            .map(|other| -> Result<String, String> {
                let info = other.iter()
                    .map(|e| {
                        self.render(e, config)
                    })
                    .reduce(|a, b| {
                        Ok(format!("{}\n{}", a?, b?))
                    })
                    .unwrap_or(Err(String::from("An error occurred while rendering personal info to markdown")));
                Ok(info?)
        });

        if let Some(other_info) = other_info {
            text = format!("{}\n{}", text, other_info?);
        }

        Ok(text)
    }
}

impl Renderer<OtherPersonalInfo, String> for MarkdownRenderer {
    fn render(self: &Self, element: &OtherPersonalInfo, _config: &Config) -> Result<String, String> {
        if let Some(url) = &element.url {
            Ok(format!("- {}: [{}]({})", element.item, url, add_https_to_url(url)))
        } else {
            Ok(format!("- {}", element.item))
        }
    }
}

impl Renderer<Objective, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Objective, config: &Config) -> Result<String, String> {
        Ok(format!("{}", split_string_across_lines(&element.objective,
                                                     config.format_config.markdown_config.width,
                                                     None,
                                                     None)))
    }
}

impl Renderer<Vec<ProfessionalExperience>, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Vec<ProfessionalExperience>, config: &Config) -> Result<String, String> {
        let mut text = format!("## Experience");

        let exp = element.iter()
            .map(|e| {
                self.render(e, config)
            })
            .reduce(|a, b| {
                Ok(format!("{}\n\n{}", a?, b?))
            })
            .unwrap_or(Err(String::from("An error occurred while rendering professional experience to markdown.")))?;

        text = format!("{}\n{}", text, exp);

        Ok(text)

    }
}

impl Renderer<ProfessionalExperience, String> for MarkdownRenderer {
    fn render(self: &Self, element: &ProfessionalExperience, config: &Config) -> Result<String, String> {
        let mut text = format!("### {}", element.organization);
        text = format!("{}\n```\n{}\n{}\n{}\n```",
                       text,
                       element.position,
                       time_range_string(&element.start, &element.end), element.location);

        let exp = element.experience.iter()
            .map(|e| {
                Ok(split_string_across_lines(e,
                                          config.format_config.markdown_config.width,
                                          Some(String::from("- ")),
                                          Some(String::from("  "))))
            })
            .reduce(|a, b| {
                Ok(format!("{}\n{}", a?, b?))
            })
            .unwrap_or(Err(String::from("An error occurred while rendering professional experience to markdown.")))?;

        text = format!("{}\n{}", text, exp);

        Ok(text)
    }
}

impl Renderer<OtherExperience, String> for MarkdownRenderer {
    fn render(self: &Self, element: &OtherExperience, config: &Config) -> Result<String, String> {
        let mut text = format!("## Projects");

        // todo: clean up?
        let projects = element.projects.iter()
            .map(|e| {
                self.render(e, config)
            })
            .reduce(|a, b| {
                Ok(format!("{}\n{}", a?, b?))
            })
            .unwrap_or(Err(format!("An error occurred while rendering other experience to markdown.")))?;

        text = format!("{}\n{}", text, projects);

        Ok(text)
    }
}

impl Renderer<ProjectInfo, String> for MarkdownRenderer {
    fn render(self: &Self, element: &ProjectInfo, config: &Config) -> Result<String, String> {
        let project_info = format!("[{}]({}) - {}",
                                   element.project_name,
                                   add_https_to_url(&element.url),
                                   element.description);
        Ok(split_string_across_lines(&project_info,
                                     config.format_config.markdown_config.width,
                                     Some(String::from("- ")),
                                     Some(String::from("  "))))
    }
}

impl Renderer<Technologies, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Technologies, _config: &Config) -> Result<String, String> {
        Ok(format!("## Technologies\n{}", element.technologies.join(", ")))
    }
}

impl Renderer<Education, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Education, _config: &Config) -> Result<String, String> {
        let mut text = format!("## University\n### {}", element.school);
        text = format!("{}\n```\n{}\n{}\n{}\n```",
                       text,
                       element.major,
                       element.graduation,
                       element.location);
        Ok(text)
    }
}

#[cfg(test)]
mod test {

}
