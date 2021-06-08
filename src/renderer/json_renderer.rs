use crate::renderer::Renderer;
use crate::data::Resume;

pub struct JsonRenderer;

impl Renderer for JsonRenderer {
    fn render(resume: Resume) -> Result<String, String> {
        serde_json::to_string(&resume).map_err(|e| {
            format!("An error occurred while trying to serialize resume as JSON: {}", e)
        })
    }
}

#[cfg(test)]
mod test {
    use crate::renderer::json_renderer::{JsonRenderer, Renderer};
    use crate::data::{Resume, PersonalInfo, Objective, ProfessionalExperience};

    #[test]
    fn test_json_renderer() {
        let resume = Resume {
            personal_info: PersonalInfo {
                name: String::from("Foo Bar"),
                email: Option::None,
                phone: Option::None,
                website: Option::None,
                other: Option::None,
            },
            objective: Objective { objective: String::from("objective") },
            professional_experience: vec![ProfessionalExperience {
                organization: String::from("organization"),
                location: String::from("location"),
                position: String::from("position"),
                start: String::from("start"),
                end: String::from("end"),
                experience: vec![String::from("experience")],
            }],
            education: Option::None,
            other_experience: Option::None,
        };
        let json = JsonRenderer::render(resume).unwrap();
        assert_eq!(json, "{\"personal_info\":{\"name\":\"Foo Bar\"},\"objective\":{\"objective\":\"objective\"},\"professional_experience\":[{\"organization\":\"organization\",\"location\":\"location\",\"position\":\"position\",\"start\":\"start\",\"end\":\"end\",\"experience\":[\"experience\"]}]}");
    }
}
