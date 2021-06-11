use std::{fs};
use serde::Deserialize;
use crate::config::Config;
use chrono::{Local, Datelike};
use std::path::{Path, PathBuf};

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

pub fn get_path(dir: Option<&String>, file_name: &str, extension: Option<&String>) -> PathBuf {
    let dir = if let Some(d) = dir {
        Path::new(d)
    } else { Path::new(".") };

    dir.join(file_name)
        .with_extension(extension.unwrap_or(&String::new()))
}

pub fn write_string_to_file(s: &str, dir: Option<&String>, file_name: &str, extension: Option<&String>) -> Result<PathBuf, String> {
    let file_path = get_path(dir, file_name, extension);
    write_string_to_path(s, &file_path)
}

pub fn write_string_to_path(s: &str, path: &Path) -> Result<PathBuf, String> {
    let dir = path.parent().unwrap_or(Path::new("."));
    fs::create_dir_all(dir).map_err(|e| {
        format!("An error occurred while creating directory [{}]: {}", dir.display(), e)
    })?;
    fs::write(&path, s).map_err(|e| {
        format!("An error occurred while writing file [{}]: {}", path.display(), e)
    })?;
    Ok(path.to_path_buf())
}

pub fn get_phone_number(phone_number: &str, config: &Config) -> Option<String> {
    if config.args.public {
        None
    } else {
        Some(phone_number.to_owned())
    }
}

pub fn time_range_string(start: &str, end: &str) -> String {
    format!("{} - {}", start, end)
}

pub fn footer_text() -> String {
    let now = Local::now();
    // todo: return the link separately so renderers can add it as a link?
    format!("Generated on {day} {month} {year} using github.com/spencewenski/resume_generator",
            day=now.day(), month=now.format("%B"), year=now.year())
}

/// Escape some special characters by placing a '\' in front
pub fn escape_special_chars(s: &str) -> String {
    // Escape '\' first, otherwise it will be escaped later after it's added to escape something else
    const SPECIAL_CHARS: [char; 10] = ['\\', '&', '%', '$', '#', '_', '{', '}', '~', '^'];
    let mut s = s.to_owned();
    SPECIAL_CHARS.iter().for_each(|x| {
        s = s.replace(&format!("{}", x), &format!("\\{}", x));
    });
    s
}
