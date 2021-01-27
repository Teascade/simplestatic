use std::fmt;
use std::io;
use std::path::PathBuf;

pub enum GenericError {
    PathError(PathError),
    IOError(io::Error),
    RegexError(regex::Error),
    StrError(String),
}

impl From<io::Error> for GenericError {
    fn from(e: io::Error) -> GenericError {
        GenericError::IOError(e)
    }
}

impl From<PathError> for GenericError {
    fn from(e: PathError) -> GenericError {
        GenericError::PathError(e)
    }
}

impl From<regex::Error> for GenericError {
    fn from(e: regex::Error) -> GenericError {
        GenericError::RegexError(e)
    }
}

impl From<&str> for GenericError {
    fn from(e: &str) -> GenericError {
        GenericError::StrError(e.to_owned())
    }
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            GenericError::PathError(e) => {
                format!("Error: {}", e)
            }
            GenericError::IOError(e) => format!("Error: {}", e),
            GenericError::RegexError(e) => {
                format!("Error: {}", e)
            }
            GenericError::StrError(e) => {
                format!("Error: {}", e)
            }
        };
        write!(f, "{}", text)
    }
}

pub struct PathError {
    pub path: PathBuf,
    pub text: String,
}

impl PathError {
    pub fn new<T: Into<String>>(path: PathBuf, text: T) -> Self {
        PathError {
            path,
            text: text.into(),
        }
    }
}

impl fmt::Display for PathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error with path {:?}: {}", self.path, self.text)
    }
}
