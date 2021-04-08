use crate::errors::GenericError;
use data_encoding::BASE64;
use minifier::{css, js};
use regex::{Captures, Regex};
use ring::digest;

#[derive(Clone)]
pub struct Template {
    text: String,
    regex: Regex,
    pub unsafe_inline: bool,
}

impl Template {
    pub fn new<T: Into<String>>(
        text: T,
        css: Option<Vec<String>>,
        js: Option<Vec<String>>,
    ) -> Result<(Self, Vec<String>, Vec<String>), GenericError> {
        let css = match css {
            Some(c) => Template::create_tags(c, Tag::Style)?,
            None => String::new(),
        };
        let js = match js {
            Some(j) => Template::create_tags(j, Tag::Script)?,
            None => String::new(),
        };

        let unsafe_inline = js.contains('\n') || css.contains('\n');

        let re = Regex::new(r"\{\{ (?P<item>.*) \}\}")?;
        let (text, js_hashes, css_hashes) = Template::initialize_text(&re, text.into(), css, js)?;

        Ok((
            Template {
                text: text,
                regex: re,
                unsafe_inline,
            },
            js_hashes,
            css_hashes,
        ))
    }

    pub fn render(&self, host: String, ua: String) -> String {
        self.text
            .replace("{{ host }}", &host)
            .replace("{{ user-agent }}", &ua)
    }

    fn create_tags(list: Vec<String>, tag: Tag) -> Result<String, GenericError> {
        let mut text = String::new();
        for item in list {
            text += &format!("<{}>{}</{}>", tag.as_str(), item, tag.as_str());
        }
        Ok(text)
    }

    fn initialize_text(
        regex: &Regex,
        html: String,
        css: String,
        js: String,
    ) -> Result<(String, Vec<String>, Vec<String>), GenericError> {
        // Add CSS and JS to the template text to specified locations
        let new_text = (*regex.replace_all(&html, |caps: &Captures| {
            match &*caps["item"].to_lowercase() {
                "css" => String::from(&css),
                "js" => String::from(&js),
                x => format!("{{{{ {} }}}}", x),
            }
        }))
        .to_owned();

        // Force the contents to be CRLF (it's a HTML standard thing)
        // Required so the digest is correct.
        /*
        let mut i = 0;
        let mut previous: Option<char> = None;
        while i < new_text.len() {
            let current = new_text.get(i..=i);
            if let Some(curr) = current {
                let first = curr.chars().nth(0).unwrap();
                if first == '\n' {
                    if let Some(prev) = previous {
                        if prev != '\r' {
                            new_text.insert(i, '\r');
                            i += 1;
                        }
                    }
                }
                previous = Some(first)
            } else {
                previous = None;
            }

            i += 1;
        }*/

        // Turns out this does not work on Linux, so as at least a temporary solution
        // Use a minifier instead.

        let new_text = Template::minimize(&new_text, Tag::Script)?;
        let new_text = Template::minimize(&new_text, Tag::Style)?;

        let js_hashes = Template::get_hashes(&new_text, Tag::Script)?;
        let css_hashes = Template::get_hashes(&new_text, Tag::Style)?;

        Ok((new_text, js_hashes, css_hashes))
    }

    fn minimize(text: &String, tag: Tag) -> Result<String, GenericError> {
        let regex = tag.as_regex()?;
        let text = (*regex.replace_all(&text.clone(), |caps: &Captures| {
            let content = match tag {
                Tag::Script => js::minify(&caps["content"]),
                Tag::Style => css::minify(&caps["content"]).unwrap_or("ERR".to_owned()),
            };
            return format!("<{}>{}</{}>", tag.as_str(), content, tag.as_str());
        }))
        .to_owned();
        Ok(text)
    }

    fn get_hashes(text: &String, tag: Tag) -> Result<Vec<String>, GenericError> {
        let mut hashes = Vec::new();
        let regex = tag.as_regex()?;
        for caps in regex.captures_iter(&text) {
            let digest = digest::digest(&digest::SHA256, caps["content"].as_bytes());
            let base64 = BASE64.encode(digest.as_ref());
            hashes.push(format!("'sha256-{}'", base64));
        }
        Ok(hashes)
    }
}

enum Tag {
    Script,
    Style,
}

impl Tag {
    pub fn as_str(&self) -> String {
        match self {
            Tag::Script => "script".to_owned(),
            Tag::Style => "style".to_owned(),
        }
    }

    pub fn as_regex(&self) -> Result<Regex, GenericError> {
        let tag = self.as_str();
        Ok(Regex::new(&format!(
            r"<\s*(?i){0}\s*>(?P<content>([\s\S]*?))<\s*/\s*{0}\s*>",
            tag
        ))?)
    }
}
