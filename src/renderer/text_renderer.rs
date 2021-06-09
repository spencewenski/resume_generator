use crate::renderer::Renderer;
use crate::data::{Resume, PersonalInfo};

pub struct TextRenderer;

impl Renderer<Resume> for TextRenderer {
    fn render(element: Resume) -> Result<String, String> {
        TextRenderer::render(element.personal_info)
    }
}

impl Renderer<PersonalInfo> for TextRenderer {
    fn render(element: PersonalInfo) -> Result<String, String> {
        let text = format!("{:^100}", element.name);
        Ok(text)
    }
}

#[cfg(test)]
mod test {
    use crate::data::PersonalInfo;
    use crate::renderer::text_renderer::TextRenderer;
    use crate::renderer::Renderer;

    #[test]
    fn test_text_renderer() {

    }

    #[test]
    fn test_personal_info_text_renderer() {
        let personal_info = PersonalInfo {
            name: String::from("Foo Bar"),
            email: Option::None,
            phone: Option::None,
            website: Option::None,
            other: Option::None,
        };
        let rendered = TextRenderer::render(personal_info).unwrap();

        assert_eq!(rendered, "                                              Foo Bar                                               ");
    }
}
