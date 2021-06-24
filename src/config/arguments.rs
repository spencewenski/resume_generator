use argparse::{ArgumentParser, Store, StoreOption};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Arguments {
    pub resume_input: String,
    pub output_name: String,
    pub cover_letter_output_name: Option<String>,
    pub output_dir: Option<String>,
    pub format_config: Option<String>,
    pub email: Option<String>,
}

impl Arguments {
    pub fn parse_args() -> Arguments {
        let mut args: Arguments = Default::default();
        {
            let mut ap = ArgumentParser::new();
            ap.set_description("Resume Generator.");

            ap.refer(&mut args.resume_input)
                .add_option(&["-i", "--resume-input"], Store, "The resume data file.")
                .required();

            ap.refer(&mut args.output_name)
                .add_option(
                    &["-o", "--output-name"],
                    Store,
                    "The name to use for the generated resumes.",
                )
                .required();

            ap.refer(&mut args.cover_letter_output_name).add_option(
                &["-c", "--cover-letter-output-name"],
                StoreOption,
                "The name to use for the generated cover letters. Defaults to appending '-cover_letter' to the output-name",
            );

            ap.refer(&mut args.output_dir)
                .add_option(&["-d", "--output-dir"],
                            StoreOption,
                            "The directory in which resume files will be placed. The directory is created if it doesn't exist Default: current directory");

            ap.refer(&mut args.format_config).add_option(
                &["-f", "--format-config"],
                StoreOption,
                "Configuration file for the various resume formats. Optional.",
            );

            ap.refer(&mut args.email).add_option(
                &["-e", "--email"],
                StoreOption,
                "Override the email in the resume config with this one.",
            );

            ap.parse_args_or_exit();
        }

        args
    }
}
