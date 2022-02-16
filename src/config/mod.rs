use crate::config::arguments::Arguments;
use crate::config::format_config::FormatConfig;
use clap::Parser;

pub mod arguments;
pub mod format_config;

#[derive(Debug, Default)]
pub struct Config {
    pub format_config: FormatConfig,
    pub args: Arguments,
}

impl Config {
    pub fn new_and_parse_args() -> Result<Config, String> {
        let args = Arguments::parse();

        let format_config = if let Some(file_name) = &args.format_config {
            FormatConfig::read_from_config_file(file_name)?
        } else {
            Default::default()
        };

        Ok(Config {
            format_config,
            args,
        })
    }
}
