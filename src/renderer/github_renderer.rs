use crate::config::Config;
use crate::data::{Objective, OtherExperience, PersonalInfo, ProjectInfo, Resume, Technologies};
use crate::renderer::markdown_renderer::MarkdownRenderer;
use crate::renderer::Renderer;
use crate::util::{add_https_to_url, write_string_to_file, FooterText};
use std::path::PathBuf;

#[derive(Default)]
pub struct GitHubRenderer {
    md: MarkdownRenderer,
}

impl GitHubRenderer {
    pub fn new() -> GitHubRenderer {
        GitHubRenderer::default()
    }
}

impl Renderer<Resume, PathBuf> for GitHubRenderer {
    fn render(&self, element: &Resume, config: &Config) -> Result<PathBuf, String> {
        let s: String = self.render(element, config)?;

        write_string_to_file(
            &s,
            config.args.output_dir.as_ref(),
            &format!("{}-github", &config.args.output_name),
            Some(String::from("md")).as_ref(),
        )
    }
}

impl Renderer<Resume, String> for GitHubRenderer {
    fn render(&self, element: &Resume, config: &Config) -> Result<String, String> {
        let mut text = format!("# {}", element.name);
        text = format!("{}\n\n{}", text, self.render(&element.objective, config)?);
        text = format!(
            "{}\n\n{}",
            text,
            self.render(&element.personal_info, config)?
        );
        if let Some(e) = &element.other_experience {
            text = format!("{}\n\n{}", text, self.render(e, config)?);
        }
        if let Some(e) = &element.technologies {
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

impl Renderer<PersonalInfo, String> for GitHubRenderer {
    fn render(&self, element: &PersonalInfo, config: &Config) -> Result<String, String> {
        let info = self.md.render(element, config)?;
        // Don't want to display email or linkedin on GitHub
        // todo: enable configuring what to display in the toml
        // todo: is there a way to do this without using 'to_owned'?
        let info = info
            .split('\n')
            .map(|x| x.to_owned())
            .filter(|x| !x.contains(&element.email))
            .filter(|x| !x.to_lowercase().contains("linkedin"))
            .collect::<Vec<String>>()
            .join("\n");
        Ok(info)
    }
}

impl Renderer<Objective, String> for GitHubRenderer {
    fn render(&self, element: &Objective, config: &Config) -> Result<String, String> {
        self.md.render(element, config)
    }
}

impl Renderer<OtherExperience, String> for GitHubRenderer {
    fn render(&self, element: &OtherExperience, config: &Config) -> Result<String, String> {
        let mut text = "## Projects".to_string();

        // todo: clean up?
        let projects = element
            .get_projects_for_github()
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

impl Renderer<ProjectInfo, String> for GitHubRenderer {
    fn render(&self, element: &ProjectInfo, config: &Config) -> Result<String, String> {
        self.md.render(element, config)
    }
}

impl Renderer<Technologies, String> for GitHubRenderer {
    fn render(&self, element: &Technologies, config: &Config) -> Result<String, String> {
        self.md.render(element, config)
    }
}

#[cfg(test)]
mod test {
    use crate::config::format_config::{FormatConfig, TextConfig};
    use crate::config::Config;
    use crate::data::{
        Objective, OtherExperience, OtherPersonalInfo, PersonalInfo, ProjectInfo, Technologies,
    };
    use crate::renderer::github_renderer::GitHubRenderer;
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
        let rendered = GitHubRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "## Find me here\n- GitHub: [github.com/foo](https://github.com/foo)\n- Foo: [example.com/foo](https://example.com/foo)\n- Bar: [example.com/bar](https://example.com/bar)"
        );
    }

    #[test]
    fn test_objective() {
        let x = Objective {
            objective: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut"),
        };
        let rendered = GitHubRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(rendered, "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut");
    }

    #[test]
    fn test_other_experience() {
        let a = ProjectInfo {
            project_name: String::from("project_nameA"),
            description: String::from("descriptionA"),
            url: String::from("example.com"),
            include_on_github: true,
            ..Default::default()
        };
        let b = ProjectInfo {
            project_name: String::from("project_nameB"),
            description: String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut"),
            url: String::from("example.com"),
            include_on_github: true,
            ..Default::default()
        };
        let c = ProjectInfo {
            project_name: String::from("project_nameC"),
            description: String::from("descriptionC"),
            url: String::from("example.com"),
            include_on_github: true,
            ..Default::default()
        };
        let x = OtherExperience {
            projects: vec![a, b, c],
        };

        let rendered = GitHubRenderer::new().render(&x, &get_config()).unwrap();

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

        let rendered = GitHubRenderer::new().render(&x, &get_config()).unwrap();

        assert_eq!(
            rendered,
            "## Technologies\nLorem, ipsum, dolor, sit, amet, consectetur, adipiscing, elit, sed, do, eiusmod, tempor, incididunt"
        )
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
