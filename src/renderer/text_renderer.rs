use crate::config::Config;
use crate::data::{
    Education, Objective, OtherExperience, PersonalInfo, ProfessionalExperience, ProjectInfo,
    Resume, Technologies,
};
use crate::renderer::Renderer;
use crate::util::{split_string_across_lines, time_range_string, write_string_to_file, FooterText};
use std::path::PathBuf;

pub struct TextRenderer;

impl TextRenderer {
    pub fn new() -> TextRenderer {
        TextRenderer
    }
}

impl Renderer<Resume, PathBuf> for TextRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<PathBuf, String> {
        let s: String = self.render(element, config)?;

        write_string_to_file(
            &s,
            config.args.output_dir.as_ref(),
            &config.args.output_name,
            Some(String::from("txt")).as_ref(),
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
        text = format!(
            "{}\n\n{footer:^width$}\n",
            text,
            footer = FooterText::new().basic_text,
            width = config.format_config.text_config.width
        );
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
        let mut text = format!(
            "{header:^width$}",
            header = "EXPERIENCE",
            width = config.format_config.text_config.width
        );

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
        let mut text = right_and_left_aligned(
            &element.organization,
            &element.location,
            config.format_config.text_config.width,
        );
        text = format!(
            "{}\n{}",
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
        let mut technologies = element.technologies.join(", ");
        technologies = centered_string(&technologies, config.format_config.text_config.width);
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

fn centered_string(s: &str, width: usize) -> String {
    format!("{s:^width$}", s = s, width = width)
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
    use crate::data::PersonalInfo;
    use crate::renderer::text_renderer::TextRenderer;
    use crate::renderer::Renderer;

    #[test]
    fn test_text_renderer() {}

    #[test]
    fn test_personal_info_text_renderer() {
        let personal_info = PersonalInfo {
            email: String::from("foo@bar.com"),
            github: String::from("github.com/foo"),
            ..Default::default()
        };
        let config = Config {
            format_config: FormatConfig {
                text_config: TextConfig { width: 50 },
                ..Default::default()
            },
            ..Default::default()
        };
        let rendered: String = TextRenderer::new().render(&personal_info, &config).unwrap();

        assert_eq!(
            rendered,
            "github.com/foo                         foo@bar.com"
        );
    }
}
