pub mod text_renderer;
pub mod json_renderer;

pub trait Renderer<T> {
    fn render(element: T) -> Result<String, String>;
}
