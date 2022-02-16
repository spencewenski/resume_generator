use clap::Parser;

/// Resume Generator.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Arguments {
    /// The resume data file.
    #[clap(short = 'i', long)]
    pub resume_input: String,

    /// The name to use for the generated resumes.
    #[clap(short, long)]
    pub output_name: String,

    /// The name to use for the generated cover letters. Defaults to appending '-cover_letter' to the output-name.
    #[clap(short, long)]
    pub cover_letter_output_name: Option<String>,

    /// The directory in which resume files will be placed. The directory is created if it doesn't exist Default: current directory.
    #[clap(short = 'd', long)]
    pub output_dir: Option<String>,

    /// Configuration file for the various resume formats. Optional.
    #[clap(short, long)]
    pub format_config: Option<String>,

    /// Override the email in the resume config with this one.
    #[clap(short, long)]
    pub email: Option<String>,
}
