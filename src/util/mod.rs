use std::{fs};
use serde::Deserialize;
use crate::config::Config;
use chrono::{Local, Datelike};

pub fn string_from_file(file_name: &str) -> Result<String, String> {
    fs::read_to_string(file_name).map_err(|e| {
        format!("An error occurred while trying to open file [{}]: {}", file_name, e)
    })
}

pub fn toml_from_string<'de, T>(x: &'de str) -> Result<T, String> where T: Deserialize<'de> {
    toml::from_str::<'de, T>(&*x).map_err(|e| {
        format!("An error occurred while parsing toml: {}", e)
    })
}

pub fn get_phone_number(phone_number: &str, config: &Config) -> Option<String> {
    if config.args.public {
        None
    } else {
        Some(phone_number.to_owned())
    }
}

pub fn footer_text() -> String {
    let now = Local::now();
    format!("Generated on {day} {month} {year} using github.com/spencewenski/resume_generator",
            day=now.day(), month=now.format("%B"), year=now.year())
}
