use crate::renderer::Renderer;
use crate::data::{Resume, PersonalInfo, Objective, ProfessionalExperience, OtherExperience, Technologies, Education};
use std::path::PathBuf;
use crate::config::Config;
use crate::util::{write_string_to_file, get_phone_number, add_https_to_url, time_range_string};
use std::fmt::Display;

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
        let mut text = self.render(&element.personal_info, config)?;
        text = format!("{}\n{}", text, self.render(&element.objective, config)?);
        text = format!("{}\n{}", text, self.render(&element.professional_experience, config)?);
        if let Some(e) = &element.other_experience {
            text = format!("{}\n{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.technologies {
            text = format!("{}\n{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.education {
            text = format!("{}\n{}", text, self.render(e, config)?);
        }
        Ok(text)
    }
}

impl Renderer<PersonalInfo, String> for MarkdownRenderer {
    fn render(self: &Self, element: &PersonalInfo, config: &Config) -> Result<String, String> {
        let mut text = format!("# {}\n\n", element.name);
        text = format!("{}- Email: {}\n", text, element.email);
        text = format!("{}- GitHub: {}\n", text, add_https_to_url(&element.github));
        let phone = get_phone_number(element.phone.as_ref(), config);
        if let Some(phone) = phone {
            text = format!("{}- Phone: {}\n", text, phone)
        }
        Ok(text)
    }
}

impl Renderer<Objective, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Objective, _config: &Config) -> Result<String, String> {
        Ok(format!("{}\n", element.objective))
    }
}

impl Renderer<Vec<ProfessionalExperience>, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Vec<ProfessionalExperience>, config: &Config) -> Result<String, String> {
        let mut text = format!("## Experience\n\n");

        let exp = element.iter()
            .map(|x| {
                self.render(x, config)
            })
            .reduce(|a, b| {
                Ok(format!("{}\n\n{}", a?, b?))
            })
            .unwrap_or(Err(String::from("An error occurred while rendering professional experience to markdown.")))?;
        text = format!("{}\n{}\n", text, exp);

        Ok(text)

    }
}

impl Renderer<ProfessionalExperience, String> for MarkdownRenderer {
    fn render(self: &Self, element: &ProfessionalExperience, _config: &Config) -> Result<String, String> {
        let mut text = format!("### {}", element.organization);
        text = format!("{}\n```\n{}\n{}\n{}\n```",
                       text,
                       element.position,
                       time_range_string(&element.start, &element.end), element.location);
        text = format!("{}\n{}", text, list(&element.experience)?);

        Ok(text)
    }
}

impl Renderer<OtherExperience, String> for MarkdownRenderer {
    fn render(self: &Self, element: &OtherExperience, _config: &Config) -> Result<String, String> {
        Ok(format!("## Projects\n{}\n", list(&element.projects)?))
    }
}

impl Renderer<Technologies, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Technologies, _config: &Config) -> Result<String, String> {
        Ok(format!("## Technologies\n{}\n", element.technologies.join(", ")))
    }
}

impl Renderer<Education, String> for MarkdownRenderer {
    fn render(self: &Self, element: &Education, _config: &Config) -> Result<String, String> {
        let mut text = format!("## University\n### {}\n", element.school);
        text = format!("{}\n```\n{}\n{}\n{}\n```",
                       text,
                       element.major,
                       element.graduation,
                       element.location);
        Ok(text)
    }
}

fn list<T>(items: &[T]) -> Result<String, String> where T: Display {
    items.iter()
        .map(|item| {
            format!("- {}", item)
        })
        .reduce(|a, b| {
            format!("{}\n{}", a, b)
        })
        .ok_or(String::from("An error occurred while creating a list of items."))
}

#[cfg(test)]
mod test {

}
