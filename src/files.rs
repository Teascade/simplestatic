use crate::config::Config;
use crate::errors::{GenericError, PathError};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use warp::http::StatusCode;
use warp::reply::Reply;

static DEFAULT_HTML: &'static str = include_str!("default.html");
static DEFAULT_MIMETYPES: &'static str = include_str!("mime.types");

#[derive(Debug, Clone)]
pub struct Mimetypes {
    map: HashMap<String, String>,
}

impl Mimetypes {
    pub fn try_fetch(path: &PathBuf) -> Result<Mimetypes, GenericError> {
        let content = fs::read_to_string(path)?;
        Mimetypes::parse(content)
    }

    fn parse(content: String) -> Result<Mimetypes, GenericError> {
        let mut map = HashMap::new();
        let rows = content.lines();
        for row in rows {
            let row = row.trim();
            if row.starts_with('#') {
                continue;
            }
            let mut parts = row.split_whitespace();
            let mimetype = parts.next();
            while let Some(ext) = parts.next() {
                map.insert(ext.to_owned(), mimetype.unwrap().to_owned());
            }
        }
        Ok(Mimetypes { map })
    }

    fn get<T: Into<String>>(&self, key: T) -> Option<String> {
        self.map.get(&key.into()).map(|x| x.clone())
    }
}

impl Default for Mimetypes {
    fn default() -> Self {
        Mimetypes::parse(DEFAULT_MIMETYPES.into()).unwrap()
    }
}

pub fn serve_file(mime_types: &Mimetypes, content_path: &PathBuf, path: String) -> Box<dyn Reply> {
    if let Ok(metadata) = fs::metadata(content_path) {
        if metadata.is_dir() {
            let new_path = content_path.join(path);

            if let Ok(metadata) = fs::metadata(&new_path) {
                if metadata.is_dir() {
                    simple_404()
                } else {
                    get_file(mime_types, &new_path)
                }
            } else {
                simple_404()
            }
        } else {
            get_file(mime_types, content_path)
        }
    } else {
        simple_404()
    }
}

fn simple_404() -> Box<dyn Reply> {
    Box::new(warp::reply::with_status(
        String::from("404"),
        StatusCode::NOT_FOUND,
    ))
}

fn get_file(mime_types: &Mimetypes, path: &PathBuf) -> Box<dyn Reply> {
    if let Ok(content) = fs::read(path) {
        let mime = path
            .extension()
            .map(|x| x.to_str())
            .flatten()
            .map(move |x| mime_types.get(x))
            .flatten()
            .unwrap_or(String::from("text/plain"));
        Box::new(warp::reply::with_header(content, "Content-Type", mime))
    } else {
        simple_404()
    }
}

pub fn get_files(
    config: &Config,
) -> Result<(String, Option<Vec<String>>, Option<Vec<String>>), GenericError> {
    if let Ok(metadata) = fs::metadata(&config.html) {
        if metadata.is_dir() {
            Err(PathError::new(
                config.html.clone(),
                "html path must not be a directory",
            ))?
        }
    };
    let html_file = match fs::read_to_string(&config.html) {
        Ok(file) => file,
        Err(e) => {
            println!("No html file found: {}. Using default html file.", e);
            String::from(DEFAULT_HTML)
        }
    };

    let css_files = if let Some(css_path) = &config.css {
        Some(handle_dir_or_file(&css_path)?)
    } else {
        None
    };
    let js_files = if let Some(js_path) = &config.js {
        Some(handle_dir_or_file(&js_path)?)
    } else {
        None
    };

    Ok((html_file, css_files, js_files))
}

pub fn handle_dir_or_file(path: &PathBuf) -> Result<Vec<String>, io::Error> {
    let metadata = fs::metadata(&path)?;
    let mut list = Vec::new();

    if metadata.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry_path = entry?.path();
            if entry_path.is_dir() {
                list.extend(handle_dir_or_file(&entry_path)?.drain(..));
            } else {
                list.push(fs::read_to_string(&entry_path)?);
            }
        }
    } else {
        list.push(fs::read_to_string(path)?);
    }

    Ok(list)
}
