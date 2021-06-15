use chrono::{Datelike, Local};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub fn string_from_file(file_name: &str) -> Result<String, String> {
    fs::read_to_string(file_name).map_err(|e| {
        format!(
            "An error occurred while trying to open file [{}]: {}",
            file_name, e
        )
    })
}

pub fn toml_from_string<'de, T>(x: &'de str) -> Result<T, String>
where
    T: Deserialize<'de>,
{
    toml::from_str::<'de, T>(&*x)
        .map_err(|e| format!("An error occurred while parsing toml: {}", e))
}

pub fn get_path(dir: Option<&String>, file_name: &str, extension: Option<&String>) -> PathBuf {
    let dir = if let Some(d) = dir {
        Path::new(d)
    } else {
        Path::new(".")
    };

    dir.join(file_name)
        .with_extension(extension.unwrap_or(&String::new()))
}

pub fn write_string_to_file(
    s: &str,
    dir: Option<&String>,
    file_name: &str,
    extension: Option<&String>,
) -> Result<PathBuf, String> {
    let file_path = get_path(dir, file_name, extension);
    write_string_to_path(s, &file_path)
}

pub fn write_string_to_path(s: &str, path: &Path) -> Result<PathBuf, String> {
    let dir = path.parent().unwrap_or(Path::new("."));
    fs::create_dir_all(dir).map_err(|e| {
        format!(
            "An error occurred while creating directory [{}]: {}",
            dir.display(),
            e
        )
    })?;
    fs::write(&path, s).map_err(|e| {
        format!(
            "An error occurred while writing file [{}]: {}",
            path.display(),
            e
        )
    })?;
    Ok(path.to_path_buf())
}

pub fn add_https_to_url(url: &str) -> String {
    if url.starts_with("https://") {
        url.to_owned()
    } else {
        format!("https://{}", url)
    }
}

pub fn time_range_string(start: &str, end: &str) -> String {
    format!("{} - {}", start, end)
}

pub struct FooterText {
    pub basic_text: String,
    pub prefix: String,
    pub url: String,
}

impl FooterText {
    pub fn new() -> FooterText {
        let now = Local::now();
        let prefix = format!(
            "Generated on {day} {month} {year} using",
            day = now.day(),
            month = now.format("%B"),
            year = now.year()
        );
        let url = String::from("github.com/spencewenski/resume_generator");
        let basic_text = format!("{} {}", prefix, url);
        FooterText {
            basic_text,
            prefix,
            url,
        }
    }
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

pub fn split_string_across_lines(
    s: &str,
    width: usize,
    first_line_prefix: Option<String>,
    line_prefix: Option<String>,
) -> String {
    let split = s.split_whitespace();
    let mut text = first_line_prefix.unwrap_or_default();
    let mut line_length = text.len();
    let mut new_line = true;
    let line_prefix = line_prefix.unwrap_or_default();
    for x in split {
        // Check if adding the next word will take us over the width limit. If so, add a new line.
        if line_length + x.len() + 1 > width {
            text.push('\n');
            text.push_str(&line_prefix);
            new_line = true;
            line_length = line_prefix.len();
        }
        // Add a space before the current word, unless it's at the start of a new line.
        if !new_line {
            text.push(' ');
            line_length += 1;
        }
        new_line = false;

        // Add the next word
        text.push_str(x);
        line_length += x.len();
    }
    text
}

#[cfg(test)]
mod test {
    use crate::util::split_string_across_lines;

    #[test]
    fn test_split_string_across_lines() {
        let s = "Foo bar baz things and stuff.";
        let s = split_string_across_lines(s, 10, None, None);

        assert_eq!(s, "Foo bar\nbaz things\nand stuff.");
    }

    #[test]
    fn test_split_string_across_lines_with_prefix() {
        let s = "Foo bar baz things and stuff.";
        let s = split_string_across_lines(s, 12, None, Some(String::from("  ")));

        assert_eq!(s, "Foo bar baz\n  things and\n  stuff.");
    }
}
