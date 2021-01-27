use crate::errors::GenericError;
use regex::{Captures, Regex};

#[derive(Clone)]
pub struct Template {
    pub text: String,
    regex: Regex,
}

impl Template {
    pub fn new<T: Into<String>>(
        text: T,
        css: Option<Vec<String>>,
        js: Option<Vec<String>>,
    ) -> Result<Self, GenericError> {
        let css = match css {
            Some(c) => Template::create_tags(c, "style"),
            None => String::new(),
        };
        let js = match js {
            Some(j) => Template::create_tags(j, "script"),
            None => String::new(),
        };

        let re = Regex::new(r"\{\{ (?P<item>.*) \}\}")?;
        let text = Template::initialize_text(&re, text.into(), css, js);

        Ok(Template {
            text: text,
            regex: re,
        })
    }

    pub fn render(&self, host: String) -> String {
        (*self.regex.replace_all(&self.text, |caps: &Captures| {
            match &*caps["item"].to_lowercase() {
                "host" => String::from(&host),
                _ => String::new(),
            }
        }))
        .to_owned()
    }

    fn create_tags<T: Into<String>>(list: Vec<String>, tag: T) -> String {
        let tag = tag.into();
        let mut text = String::new();
        for item in list {
            text += &format!("<{}>", tag);
            text += &item;
            text += &format!("</{}>", tag);
        }
        text
    }

    fn initialize_text(regex: &Regex, html: String, css: String, js: String) -> String {
        // Add CSS and JS to the template text to specified locations
        let mut new_text = (*regex.replace_all(&html, |caps: &Captures| {
            match &*caps["item"].to_lowercase() {
                "css" => String::from(&css),
                "js" => String::from(&js),
                x => format!("{{{{ {} }}}}", x),
            }
        }))
        .to_owned();

        // Force the contents to be CRLF (it's a HTML standard thing)
        // Required so the digest is correct.
        let mut i = 0;
        let mut previous: Option<char> = None;
        while i < new_text.len() {
            let list: Vec<char> = new_text.chars().collect();
            let current = list.get(i..=i);
            if let Some(curr) = current {
                if curr[0] == '\n' {
                    if let Some(prev) = previous {
                        if prev != '\r' {
                            new_text.insert(i, '\r');
                            i += 1;
                        }
                    }
                }
                previous = Some(curr[0])
            } else {
                previous = None;
            }

            i += 1;
        }

        let js_regex = Regex::new(r"<\s*(?i)script\s*>(?P<content>.|(\r\n))*<\s*\/\s*script\s*>");

        new_text
    }
}
