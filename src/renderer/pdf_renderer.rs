use crate::config::Config;
use crate::data::{
    Education, Objective, OtherExperience, PersonalInfo, ProfessionalExperience, ProjectInfo,
    Resume, Technologies,
};
use crate::renderer::Renderer;
use crate::util::{
    escape_special_chars, get_path, time_range_string, write_string_to_path, FooterText,
};
use latex::{print, Document, Element, Paragraph, PreambleElement};
use std::borrow::BorrowMut;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct PdfRenderer;

impl PdfRenderer {
    pub fn new() -> PdfRenderer {
        PdfRenderer
    }
}

impl Renderer<Resume, PathBuf> for PdfRenderer {
    /// Write the LaTeX to a file, then run a command to generate a pdf from the LaTeX file
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<PathBuf, String> {
        let s: String = self.render(element, config)?;

        let path = get_path(
            config.args.output_dir.as_ref(),
            &config.args.output_name,
            Some(String::from("tex")).as_ref(),
        );
        write_string_to_path(&s, &path)?;

        let x = Command::new("pdflatex")
            .arg("-output-directory")
            .arg(path.parent().unwrap_or(Path::new(".")).as_os_str())
            .arg(path.as_os_str())
            .output()
            .map_err(|e| {
                format!(
                    "An error occurred while running the pdflatex command: {}",
                    e
                )
            })?;
        if !x.status.success() {
            Err(format!(
                "An error occurred while running the pdflatex command: {:?}",
                x
            ))
        } else {
            Ok(path.with_extension("pdf"))
        }
    }
}

impl Renderer<Resume, String> for PdfRenderer {
    /// Render a LaTeX string
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<String, String> {
        // Build the document
        let doc: Document = self.render(element, config)?;

        // convert to a string
        print(&doc).map_err(|e| {
            format!(
                "An error occurred while rendering the LaTeX document to a string: {}",
                e
            )
        })
    }
}

impl Renderer<Resume, Document> for PdfRenderer {
    fn render(self: &Self, element: &Resume, config: &Config) -> Result<Document, String> {
        let mut doc = Document::default();
        // LaTeX preamble
        {
            doc.preamble
                // Set the margins
                .push(PreambleElement::UsePackage {
                    package: String::from("geometry"),
                    argument: Some(format!(
                        "margin={}",
                        &config.format_config.pdf_config.margin
                    )),
                })
                // Set up the font encoding
                .push(PreambleElement::UsePackage {
                    package: String::from("fontenc"),
                    argument: Some(String::from("T1")),
                })
                // Set up the footer and remove the header
                .use_package("fancyhdr")
                .push(PreambleElement::UserDefined(String::from("\\fancyhf{}")))
                .push(PreambleElement::UserDefined(String::from(
                    r"\pagestyle{fancy}",
                )))
                .push(PreambleElement::UserDefined(String::from(
                    r"\renewcommand{\headrulewidth}{0pt}",
                )))
                .push(PreambleElement::UserDefined(String::from(format!(
                    "\\cfoot{{{}}}",
                    escape_special_chars(&FooterText::new().basic_text)
                ))));
        }

        // Add the actual resume content
        {
            // Name
            doc.push(Element::Environment(
                String::from("center"),
                vec![format!("\\bf\\Large {}", element.name)],
            ));

            // Header
            doc.push_doc(&self.render(&element.personal_info, config)?);

            // We want everything to be flush to the left side (except for a few outliers)
            doc.push(Element::UserDefined(String::from("\\begin{flushleft}")));
            {
                doc.push_doc(&self.render(&element.objective, config)?);
                doc.push_doc(&vspace());
                doc.push_doc(&self.render(&element.professional_experience, config)?);
                if let Some(e) = &element.other_experience {
                    doc.push_doc(&vspace());
                    doc.push_doc(&self.render(e, config)?);
                }
                if let Some(e) = &element.technologies {
                    doc.push_doc(&vspace());
                    doc.push_doc(&self.render(e, config)?);
                }
                if let Some(e) = &element.education {
                    doc.push_doc(&vspace());
                    doc.push_doc(&self.render(e, config)?);
                }
            }

            doc.push(Element::UserDefined(String::from("\\end{flushleft}")));
        }

        Ok(doc)
    }
}

impl Renderer<PersonalInfo, Document> for PdfRenderer {
    fn render(self: &Self, element: &PersonalInfo, _config: &Config) -> Result<Document, String> {
        let mut doc = Document::default();
        // todo: display something else instead of phone number?
        doc.push(Element::UserDefined(format!(
            "{} \\hfill {}",
            element.github, element.email
        )));
        doc.push(Element::UserDefined(String::from(
            "\\rule{\\textwidth}{0.4pt}",
        )));

        Ok(doc)
    }
}

impl Renderer<Objective, Document> for PdfRenderer {
    fn render(self: &Self, element: &Objective, _config: &Config) -> Result<Document, String> {
        let mut doc = Document::default();
        doc.push(Paragraph::from(element.objective.as_str()));
        Ok(doc)
    }
}

impl Renderer<Vec<ProfessionalExperience>, Document> for PdfRenderer {
    fn render(
        self: &Self,
        element: &Vec<ProfessionalExperience>,
        config: &Config,
    ) -> Result<Document, String> {
        let mut doc = Document::default();
        doc.push_doc(&section_header("EXPERIENCE"));

        let reduced = element
            .iter()
            .map(|x| self.render(x, config))
            .reduce(|a, b| {
                let mut a = a?;
                a.push(Element::UserDefined(String::from(
                    "\\vspace*{\\baselineskip}",
                )));
                a.push_doc(&b?);
                Ok(a)
            })
            .unwrap_or(Err(format!(
                "An error occurred while rendering professional experience to LaTeX."
            )))?;
        doc.push_doc(&reduced);

        Ok(doc)
    }
}

impl Renderer<ProfessionalExperience, Document> for PdfRenderer {
    fn render(
        self: &Self,
        element: &ProfessionalExperience,
        _config: &Config,
    ) -> Result<Document, String> {
        let mut doc = Document::default();
        doc.push(Element::UserDefined(format!(
            "{{\\bf {}}} \\hfill {}\n",
            element.organization, element.location
        )));
        doc.push(Element::UserDefined(format!(
            "\\emph{{{}}} \\hfill {}\n",
            element.position,
            time_range_string(&element.start, &element.end)
        )));
        let mut itemize_content = vec![String::from("\\setlength\\itemsep{-0.05in}")];
        let mut exp = element
            .experience
            .iter()
            .map(|e| format!("\\item {}", e))
            .collect::<Vec<String>>();
        itemize_content.append(exp.borrow_mut());
        doc.push(Element::Environment(
            String::from("itemize"),
            itemize_content,
        ));
        Ok(doc)
    }
}

impl Renderer<OtherExperience, Document> for PdfRenderer {
    fn render(self: &Self, element: &OtherExperience, config: &Config) -> Result<Document, String> {
        let mut doc = Document::default();
        doc.push_doc(&section_header("PROJECTS"));
        // todo: clean up?
        // let mut itemize_content = vec![String::from("\\setlength\\itemsep{-0.05in}")];
        // let mut projects = element.projects.iter()
        //     .map(|p| {
        //         self.render(p, config)
        //     })
        //     .reduce(|a, b| {
        //         let mut a = a?;
        //         a.push_doc(&b?);
        //         Ok(a)
        //     }).unwrap_or(Err(format!("An error occurred while rendering other experience to LaTeX.")))?;
        // itemize_content.append(projects.borrow_mut());
        // doc.push_doc(&itemize_content);

        let mut itemize_content = vec![String::from("\\setlength\\itemsep{-0.05in}")];
        let mut projects = element
            .projects
            .iter()
            .map(|e| self.render(e, config).unwrap_or_default())
            .collect::<Vec<String>>();
        itemize_content.append(projects.borrow_mut());
        doc.push(Element::Environment(
            String::from("itemize"),
            itemize_content,
        ));

        Ok(doc)
    }
}

impl Renderer<ProjectInfo, String> for PdfRenderer {
    fn render(self: &Self, element: &ProjectInfo, _config: &Config) -> Result<String, String> {
        Ok(format!("\\item {}\n", element.description))
    }
}

impl Renderer<Technologies, Document> for PdfRenderer {
    fn render(self: &Self, element: &Technologies, _config: &Config) -> Result<Document, String> {
        let mut doc = Document::default();
        doc.push_doc(&section_header("TECHNOLOGIES"));
        let technologies = element.technologies.join(", ");
        let technologies = escape_special_chars(&technologies);
        doc.push(Element::Environment(
            String::from("center"),
            vec![technologies],
        ));
        Ok(doc)
    }
}

impl Renderer<Education, Document> for PdfRenderer {
    fn render(self: &Self, element: &Education, _config: &Config) -> Result<Document, String> {
        let mut doc = Document::default();
        doc.push_doc(&section_header("UNIVERSITY"));
        doc.push(Element::UserDefined(format!(
            "{{\\bf {}}} \\hfill {}\n",
            element.school, element.location
        )));
        doc.push(Element::UserDefined(format!(
            "\\emph{{{}}} \\hfill {}\n",
            element.major, element.graduation
        )));
        Ok(doc)
    }
}

fn vspace() -> Document {
    let mut doc = Document::default();
    doc.push(Element::UserDefined(String::from(
        "\\vspace*{\\baselineskip}",
    )));
    doc
}

fn section_header(header: &str) -> Document {
    let mut doc = Document::default();
    doc.push(Element::Environment(
        String::from("center"),
        vec![format!("{{\\bf {}}}", header)],
    ));
    doc
}

#[cfg(test)]
mod test {}
