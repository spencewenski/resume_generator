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
        TextConfig {
            width: 100,
        }
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
        MarkdownConfig {
            width: 120,
        }
    }
}
