use crate::config::Config;
use crate::data::Resume;
use crate::renderer::github_renderer::GitHubRenderer;
use crate::renderer::markdown_renderer::MarkdownRenderer;
use crate::renderer::pdf_renderer::PdfRenderer;
use crate::renderer::text_renderer::TextRenderer;
use std::path::PathBuf;

pub mod github_renderer;
pub mod markdown_renderer;
pub mod pdf_renderer;
pub mod text_renderer;

trait Renderer<I, O> {
    fn render(&self, element: &I, config: &Config) -> Result<O, String>;
}

pub fn render_resume(resume: &Resume, config: &Config) -> Result<(), String> {
    let renderers: Vec<Box<dyn Renderer<Resume, PathBuf>>> = vec![
        Box::new(TextRenderer::default()),
        Box::new(PdfRenderer::default()),
        Box::new(MarkdownRenderer::default()),
        Box::new(GitHubRenderer::default()),
    ];
    renderers.iter().try_for_each(|x| -> Result<(), String> {
        x.render(resume, config)?;
        Ok(())
    })
}
