use crate::renderer::Renderer;
use crate::data::{Resume, PersonalInfo, Objective, OtherExperience, Technologies};
use std::path::PathBuf;
use crate::config::Config;
use crate::util::{write_string_to_file};
use crate::renderer::markdown_renderer::MarkdownRenderer;

pub struct GitHubRenderer {
    md: MarkdownRenderer,
}

impl GitHubRenderer {
    pub fn new() -> GitHubRenderer {
        GitHubRenderer {
            md: MarkdownRenderer::new(),
        }
    }
}

impl Renderer<Resume, PathBuf> for GitHubRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<PathBuf, String> {
        let s: String = self.render(element, config)?;

        write_string_to_file(&s,
                             config.args.output_dir.as_ref(),
                             &format!("{}-github", &config.args.output_name),
                             Some(String::from("md")).as_ref())
    }
}

impl Renderer<Resume, String> for GitHubRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<String, String> {
        let mut text = format!("# {}", element.name);
        text = format!("{}\n\n{}", text, self.render(&element.objective, config)?);
        text = format!("{}\n\n{}", text, self.render(&element.personal_info, config)?);
        if let Some(e) = &element.other_experience {
            text = format!("{}\n\n{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.technologies {
            text = format!("{}\n\n{}", text, self.render(e, config)?);
        }
        Ok(text)
    }
}

impl Renderer<PersonalInfo, String> for GitHubRenderer {
    fn render(self: &Self, element: &PersonalInfo, config: &Config) -> Result<String, String> {
        let info = self.md.render(element, config)?;
        // Don't want to display email on GitHub
        // todo: is there a way to do this without using 'to_owned'?
        let info = info.split("\n")
            .map(|x| { x.to_owned()})
            .filter(|x| {
                !x.contains(&element.email)
            })
            .filter(|x| {
                !x.to_lowercase().contains("linkedin")
            })
            .collect::<Vec<String>>()
            .join("\n");
        Ok(info)
    }
}

impl Renderer<Objective, String> for GitHubRenderer {
    fn render(self: &Self, element: &Objective, config: &Config) -> Result<String, String> {
        self.md.render(element, config)
    }
}

impl Renderer<OtherExperience, String> for GitHubRenderer {
    fn render(self: &Self, element: &OtherExperience, config: &Config) -> Result<String, String> {
        self.md.render(element, config)
    }
}

impl Renderer<Technologies, String> for GitHubRenderer {
    fn render(self: &Self, element: &Technologies, config: &Config) -> Result<String, String> {
        self.md.render(element, config)
    }
}
