#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FormatConfig {
    pub text_config: TextConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextConfig {
    pub width: usize,
}

impl Default for TextConfig {
    fn default() -> Self {
        TextConfig {
            width: 100,
        }
    }
}
