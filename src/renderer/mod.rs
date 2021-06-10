use crate::data::Resume;
use crate::renderer::text_renderer::TextRenderer;
use crate::config::Config;
use std::fs;
use std::path::Path;

pub mod text_renderer;

trait Renderer<T> {
    fn file_extension(self: &Self) -> String { Default::default() }

    fn render_to_string(self: &Self, element: &T, config: &Config) -> Result<String, String>;

    fn render_to_file(self: &Self, element: &T, config: &Config) -> Result<(), String> {
        let s = self.render_to_string(element, config)?;

        let dir = if let Some(d) = &config.args.output_dir {
            let p = Path::new(d);
            fs::create_dir_all(p).map_err(|e| {
                format!("An error occurred while creating directory [{}]: {}", d, e)
            })?;
            p
        } else { Path::new(".") };

        let dir = dir.join(&config.args.output_name)
            .with_extension(self.file_extension());
        fs::write(&dir, s).map_err(|e| {
            format!("An error occurred while writing file [{}]: {}", dir.display(), e)
        })
    }
}

pub fn render_resume(resume: &Resume, config: &Config) -> Result<(), String> {
    let renderers = vec![Box::new(TextRenderer::new())];
    renderers.iter().try_for_each(|x| -> Result<(), String> {
        x.render_to_file(resume, config)
    })
}
