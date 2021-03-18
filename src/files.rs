use crate::config::Config;
use crate::errors::{GenericError, PathError};
use std::fs;
use std::io;
use std::path::PathBuf;

static DEFAULT_HTML: &'static str = include_str!("default.html");

pub fn serve_file(content_path: &PathBuf, path: String) -> String {
    String::from("Hello!")
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
