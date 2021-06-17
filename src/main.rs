extern crate resume_generator;

use resume_generator::config::Config;
use resume_generator::data::Resume;
use resume_generator::renderer::render_resume;

fn main() {
    let r = Config::new_and_parse_args().and_then(|config| {
        let resume = Resume::read_from_config_file(&config.args.resume_input, &config)?;
        render_resume(&resume, &config)
    });
    if let Err(e) = r {
        println!();
        println!("An error occurred, please try again.");
        println!("Error: {}", e);
    }
}
