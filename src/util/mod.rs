use crate::config::Config;
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
    toml::from_str::<'de, T>(x).map_err(|e| format!("An error occurred while parsing toml: {}", e))
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
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(dir).map_err(|e| {
        format!(
            "An error occurred while creating directory [{}]: {}",
            dir.display(),
            e
        )
    })?;
    fs::write(path, s).map_err(|e| {
        format!(
            "An error occurred while writing file [{}]: {}",
            path.display(),
            e
        )
    })?;
    Ok(path.to_path_buf())
}

pub fn cover_letter_file_name(config: &Config) -> String {
    if let Some(cover_letter_name) = &config.args.cover_letter_output_name {
        cover_letter_name.clone()
    } else {
        format!("{}-cover_letter", config.args.output_name)
    }
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
        let prefix = format!("Updated on {} using", date_string());
        // todo: make this url configurable somehow
        let url = String::from("github.com/spencewenski/resume_generator");
        let basic_text = format!("{} {}", prefix, url);
        FooterText {
            basic_text,
            prefix,
            url,
        }
    }
}

impl Default for FooterText {
    fn default() -> Self {
        FooterText::new()
    }
}

pub fn date_string() -> String {
    let now = Local::now();
    format!(
        "{day} {month} {year}",
        day = now.day(),
        month = now.format("%B"),
        year = now.year()
    )
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

pub fn default_true() -> bool {
    true
}

pub fn default_false() -> bool {
    false
}

#[cfg(test)]
mod test {
    use crate::config::arguments::Arguments;
    use crate::config::Config;
    use crate::util::{
        add_https_to_url, cover_letter_file_name, escape_special_chars, get_path,
        split_string_across_lines, string_from_file, time_range_string, toml_from_string,
        FooterText,
    };
    use chrono::{Datelike, Local};

    #[test]
    fn test_string_from_file() {
        let s = string_from_file("tst/test_read_file_to_string.txt").unwrap();
        assert_eq!(s, "foo\nbar\nbaz\n");
    }

    #[test]
    fn test_string_from_file_does_not_exist() {
        let s = string_from_file("tst/does_not_exist.txt");
        assert!(s.is_err());
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct TestToml {
        pub foo: String,
        pub baz: Baz,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    pub struct Baz {
        pub things: String,
    }

    #[test]
    fn test_toml_from_string() {
        let s = string_from_file("tst/test_simple_toml.toml").unwrap();
        let t: TestToml = toml_from_string(&*s).unwrap();

        assert_eq!(t.foo, String::from("bar"));
        assert_eq!(t.baz.things, String::from("stuff"));
    }

    #[test]
    fn test_get_path() {
        let p = get_path(None, "foo", None);
        assert_eq!(p.as_os_str(), "./foo");

        let dir = Some(String::from("./foo"));
        let p = get_path(dir.as_ref(), "bar", None);
        assert_eq!(p.as_os_str(), "./foo/bar");

        let ext = Some(String::from("baz"));
        let p = get_path(dir.as_ref(), "bar", ext.as_ref());
        assert_eq!(p.as_os_str(), "./foo/bar.baz");
    }

    #[test]
    fn test_cover_letter_file_name() {
        let args = Arguments {
            resume_input: String::from("foo"),
            output_name: String::from("bar"),
            ..Default::default()
        };
        let c = Config {
            args,
            ..Default::default()
        };

        let name = cover_letter_file_name(&c);
        assert_eq!(name, String::from("bar-cover_letter"));
    }

    #[test]
    fn test_cover_letter_file_name_with_arg() {
        let args = Arguments {
            resume_input: String::from("foo"),
            output_name: String::from("bar"),
            cover_letter_output_name: Some(String::from("baz")),
            ..Default::default()
        };
        let c = Config {
            args,
            ..Default::default()
        };

        let name = cover_letter_file_name(&c);
        assert_eq!(name, String::from("baz"));
    }

    #[test]
    fn test_add_https_to_url() {
        let url = add_https_to_url("example.com");
        assert_eq!(url, "https://example.com");

        let url = add_https_to_url("https://example.com");
        assert_eq!(url, "https://example.com");
    }

    #[test]
    fn test_time_range_string() {
        let s = time_range_string("foo", "bar");
        assert_eq!(s, "foo - bar");
    }

    #[test]
    fn test_footer_text() {
        let f = FooterText::new();
        assert_eq!(f.url, "github.com/spencewenski/resume_generator");
        let now = Local::now();
        let prefix = format!(
            "Updated on {day} {month} {year} using",
            day = now.day(),
            month = now.format("%B"),
            year = now.year()
        );
        assert_eq!(f.prefix, prefix);
        assert_eq!(f.basic_text, format!("{} {}", prefix, f.url));
    }

    #[test]
    fn test_escape_special_chars() {
        // '\\', '&', '%', '$', '#', '_', '{', '}', '~', '^'
        let input = "\\ & % $ # _ { } ~ ^";
        let output = escape_special_chars(input);
        assert_eq!(output, "\\\\ \\& \\% \\$ \\# \\_ \\{ \\} \\~ \\^");
    }

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

    #[test]
    fn test_split_string_across_lines_with_first_line_prefix() {
        let s = "Foo bar baz things and stuff.";
        let s =
            split_string_across_lines(s, 20, Some(String::from("- ")), Some(String::from("  ")));

        assert_eq!(s, "- Foo bar baz things\n  and stuff.");
    }
}
