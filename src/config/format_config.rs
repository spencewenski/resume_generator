#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FormatConfig {
    pub text_config: TextConfig,
    pub pdf_config: PdfConfig,
    pub markdown_config: MarkdownConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextConfig {
    pub width: usize,
}

impl Default for TextConfig {
    fn default() -> Self {
        TextConfig { width: 100 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PdfConfig {
    pub margin: String,
}

impl Default for PdfConfig {
    fn default() -> Self {
        PdfConfig {
            margin: String::from("0.75in"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkdownConfig {
    pub width: usize,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        MarkdownConfig { width: 120 }
    }
}

#[cfg(test)]
mod test {
    use crate::config::format_config::FormatConfig;

    #[test]
    fn test_format_config_defaults() {
        let c = FormatConfig::default();
        assert_eq!(c.text_config.width, 100);
        assert_eq!(c.pdf_config.margin, "0.75in");
        assert_eq!(c.markdown_config.width, 120);
    }
}
