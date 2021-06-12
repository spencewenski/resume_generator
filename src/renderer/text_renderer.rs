use crate::renderer::Renderer;
use crate::data::{Resume, PersonalInfo, Objective, ProfessionalExperience, OtherExperience, Technologies, Education};
use crate::config::Config;
use crate::util::{get_phone_number, footer_text, write_string_to_file, time_range_string};
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

        write_string_to_file(&s,
                             config.args.output_dir.as_ref(),
                             &config.args.output_name,
                             Some(String::from("txt")).as_ref())
    }
}

impl Renderer<Resume, String> for TextRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<String, String> {
        let mut text = self.render(&element.personal_info, &config)?;
        text = format!("{}{}", text, self.render(&element.objective, &config)?);
        text = format!("{}{}", text, self.render(&element.professional_experience, config)?);
        if let Some(e) = &element.other_experience {
            text = format!("{}{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.technologies {
            text = format!("{}{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.education {
            text = format!("{}{}", text, self.render(e, config)?);
        }
        text = format!("{}{footer:^width$}\n", text,
                       footer=footer_text(),
                       width=config.format_config.text_config.width);
        Ok(text)
    }
}

impl Renderer<PersonalInfo, String> for TextRenderer {
    fn render(self: &Self, element: &PersonalInfo, config: &Config) -> Result<String, String> {
        let mut text = format!("{a:^width$}\n\n", width=config.format_config.text_config.width, a=element.name);

        let space_taken = element.github.len() + element.email.len();
        if space_taken > config.format_config.text_config.width {
            return Err(format!("The given width is not wide enough to fit the email and github link."));
        }

        let middle_space = config.format_config.text_config.width - space_taken;
        let phone_number = get_phone_number(element.phone.as_ref(), config);
        let middle_space_needed = if let Some(p) = &phone_number {
            p.len() + 2
        } else {
            0
        };
        if middle_space_needed > middle_space {
            return Err(format!("The given width is not wide enough to fit the phone number in the middle."));
        }

        text = format!("{prev}{github}{phone_number:^middle$}{email}\n\n",
                       prev=text,
                       github=element.github,
                       phone_number= phone_number.unwrap_or_default(),
                       middle=middle_space,
                       email=element.email);

        Ok(text)
    }
}

impl Renderer<Objective, String> for TextRenderer {
    fn render(self: &Self, element: &Objective, config: &Config) -> Result<String, String> {
        let mut text = split_string_across_lines(&element.objective, config.format_config.text_config.width);
        text = format!("{}\n\n", text);
        Ok(text)
    }
}

impl Renderer<Vec<ProfessionalExperience>, String> for TextRenderer {
    fn render(self: &Self, element: &Vec<ProfessionalExperience>, config: &Config) -> Result<String, String> {
        let mut text = format!("{header:^width$}\n",
                               header="EXPERIENCE",
                               width=config.format_config.text_config.width);

        for experience in element {
            text = format!("{}{}\n", text, self.render(experience, &config)?);
        }

        Ok(text)
    }
}

impl Renderer<ProfessionalExperience, String> for TextRenderer {
    fn render(self: &Self, element: &ProfessionalExperience, config: &Config) -> Result<String, String> {
        let mut text = right_and_left_aligned(&element.organization,
                                              &element.location,
                                              config.format_config.text_config.width);
        text = format!("{}\n{}\n", text, right_and_left_aligned(&element.position,
                                                                &time_range_string(&element.start, &element.end),
                                                                config.format_config.text_config.width));

        for e in element.experience.iter() {
            // todo: handle long lines?
            text = format!("{prev}- {experience}\n", prev=text, experience=e);
        }

        Ok(text)
    }
}

impl Renderer<OtherExperience, String> for TextRenderer {
    fn render(self: &Self, element: &OtherExperience, config: &Config) -> Result<String, String> {
        let header = centered_string("PROJECTS", config.format_config.text_config.width);
        // todo: handle long lines?
        let projects = element.projects.iter().map(|s| -> String {
            format!("- {}", s)
        })
            .collect::<Vec<String>>()
            .join("\n");
        Ok(format!("{}\n{}\n\n", header, projects))
    }
}

impl Renderer<Technologies, String> for TextRenderer {
    fn render(self: &Self, element: &Technologies, config: &Config) -> Result<String, String> {
        let header = centered_string("TECHNOLOGIES", config.format_config.text_config.width);
        let mut technologies = element.technologies.join(", ");
        technologies = centered_string(&technologies, config.format_config.text_config.width);
        Ok(format!("{}\n{}\n\n", header, technologies))
    }
}

impl Renderer<Education, String> for TextRenderer {
    fn render(self: &Self, element: &Education, config: &Config) -> Result<String, String> {
        let mut text = centered_string("UNIVERSITY", config.format_config.text_config.width);
        text = format!("{}\n{}", text,  right_and_left_aligned(&element.school,
                                                               &element.location,
                                                               config.format_config.text_config.width));
        text = format!("{}\n{}", text,  right_and_left_aligned(&element.major,
                                                               &element.graduation,
                                                               config.format_config.text_config.width));
        text = format!("{}\n\n", text);
        Ok(text)
    }
}

fn centered_string(s: &str, width: usize) -> String {
    format!("{s:^width$}", s=s, width=width)
}

fn right_and_left_aligned(l: &str, r: &str, width: usize) -> String {
    format!("{left}{right:>width$}",
            left=l,
            right=r,
            width=width - l.len())
}

fn split_string_across_lines(s: &str, width: usize) -> String {
    let split = s.split_whitespace();
    let mut text = String::new();
    let mut line_length = 0;
    let mut new_line = true;
    for x in split {
        // Check if adding the next word will take us over the width limit. If so, add a new line.
        if line_length + x.len() + 1 > width {
            text.push('\n');
            new_line = true;
            line_length = 0;
        }
        // Add a space before the current word, unless it's at the start of a new line.
        if !new_line {
            text.push(' ');
            line_length += 1;
        }
        new_line = false;

        // Add the next word
        text.push_str(x);
        line_length += x.len();
    }
    text
}

#[cfg(test)]
mod test {
    use crate::data::{PersonalInfo};
    use crate::renderer::text_renderer::{TextRenderer, split_string_across_lines};
    use crate::renderer::Renderer;
    use crate::config::format_config::{TextConfig, FormatConfig};
    use crate::config::Config;

    #[test]
    fn test_text_renderer() {

    }

    #[test]
    fn test_personal_info_text_renderer() {
        let personal_info = PersonalInfo {
            name: String::from("Foo Bar"),
            email: String::from("foo@bar.com"),
            phone: Some(String::from("1-555-555-5555")),
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

        assert_eq!(rendered, "                     Foo Bar                      \n\ngithub.com/foo     1-555-555-5555      foo@bar.com\n\n");
    }

    #[test]
    fn test_split_string_across_lines() {
        let s = "Foo bar baz things and stuff.";
        let s = split_string_across_lines(s, 10);

        assert_eq!(s, "Foo bar
baz things
and stuff.");
    }
}
