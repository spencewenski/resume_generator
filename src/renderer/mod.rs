use crate::data::Resume;
use crate::renderer::text_renderer::TextRenderer;
use crate::config::Config;
use crate::renderer::pdf_renderer::PdfRenderer;
use std::path::PathBuf;
use crate::renderer::markdown_renderer::MarkdownRenderer;
use crate::renderer::github_renderer::GitHubRenderer;

pub mod github_renderer;
pub mod markdown_renderer;
pub mod text_renderer;
pub mod pdf_renderer;

trait Renderer<I, O> {
    fn render(self: &Self, element: &I, config: &Config) -> Result<O, String>;
}

pub fn render_resume(resume: &Resume, config: &Config) -> Result<(), String> {
    let renderers : Vec<Box<dyn Renderer<Resume, PathBuf>>> = vec![Box::new(TextRenderer::new()),
                                                                   Box::new(PdfRenderer::new()),
                                                                   Box::new(MarkdownRenderer::new()),
                                                                   Box::new(GitHubRenderer::new())];
    renderers.iter().try_for_each(|x| -> Result<(), String> {
        x.render(resume, config)?;
        Ok(())
    })
}
