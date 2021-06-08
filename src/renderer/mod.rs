use crate::data::Resume;

pub mod json_renderer;

pub trait Renderer {
    fn render(resume: Resume) -> Result<String, String>;
}
