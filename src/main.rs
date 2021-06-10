extern crate resume_generator;

use resume_generator::config::Config;
use resume_generator::data::Resume;
use resume_generator::renderer::render_resume;

fn main() {
    let r = Config::new_and_parse_args().and_then(|c| {
        let resume = Resume::read_from_config_file(&c.args.resume_input)?;
        render_resume(&resume, &c)
    });
    if let Err(e) = r {
        println!();
        println!("An error occurred, please try again.");
        println!("Error: {}", e);
    }
}
