use crate::args::MainArgs;
use std::env;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub html: PathBuf,
    pub css: Option<PathBuf>,
    pub js: Option<PathBuf>,
    pub unsafe_inline: bool,
    pub host: String,
    pub port: u16,
    pub static_path: Option<String>,
    pub static_content: PathBuf,
    pub mime_types: PathBuf,
}

pub struct ConfigBuilder {
    html: Option<PathBuf>,
    css: Option<PathBuf>,
    js: Option<PathBuf>,
    unsafe_inline: Option<bool>,
    host: Option<String>,
    port: Option<u16>,
    static_path: Option<String>,
    static_content: Option<PathBuf>,
    mime_types: Option<PathBuf>,
}

impl ConfigBuilder {
    pub fn build(&self) -> Result<Config, ()> {
        if self.html.is_none()
            || self.unsafe_inline.is_none()
            || self.host.is_none()
            || self.port.is_none()
            || self.static_content.is_none()
        {
            Err(())
        } else {
            Ok(Config {
                html: self.html.clone().unwrap(),
                css: self.css.clone(),
                js: self.js.clone(),
                unsafe_inline: self.unsafe_inline.unwrap(),
                host: self.host.clone().unwrap(),
                port: self.port.unwrap(),
                static_path: self.static_path.clone(),
                static_content: self.static_content.clone().unwrap(),
                mime_types: self.mime_types.clone().unwrap(),
            })
        }
    }

    pub fn or_from_cmd(&self) -> ConfigBuilder {
        let args: MainArgs = argh::from_env();
        self.or_rather(ConfigBuilder {
            html: args.html,
            css: args.css,
            js: args.js,
            unsafe_inline: if args.unsafe_inline { Some(true) } else { None },
            host: args.host,
            port: args.port,
            static_path: args.static_path,
            static_content: args.static_content,
            mime_types: args.mime_types,
        })
    }

    pub fn or_from_env(&self) -> ConfigBuilder {
        self.or_rather(ConfigBuilder {
            html: env::var("SSTATIC_HTML_PATH").ok().map(PathBuf::from),
            css: env::var("SSTATIC_JS_PATH").ok().map(PathBuf::from),
            js: env::var("SSTATIC_CSS_PATH").ok().map(PathBuf::from),
            unsafe_inline: env::var("SSTATIC_UNSAFE_INLINE").ok().map(|_| true),
            host: env::var("SSTATIC_HOST").ok().map(String::from),
            port: env::var("SSTATIC_PORT")
                .ok()
                .map(|x| x.parse::<u16>().unwrap_or(3333)),
            static_path: env::var("SSTATIC_STATIC_PATH").ok().map(String::from),
            static_content: env::var("SSTATIC_STATIC_CONTENT").ok().map(PathBuf::from),
            mime_types: env::var("SSTATIC_MIME_TYPES").ok().map(PathBuf::from),
        })
    }

    pub fn or_rather(&self, other: ConfigBuilder) -> ConfigBuilder {
        ConfigBuilder {
            html: other.html.or(self.html.clone()),
            css: other.css.or(self.css.clone()),
            js: other.js.or(self.js.clone()),
            unsafe_inline: other.unsafe_inline.or(self.unsafe_inline),
            host: other.host.or(self.host.clone()),
            port: other.port.or(self.port),
            static_path: other.static_path.or(self.static_path.clone()),
            static_content: other.static_content.or(self.static_content.clone()),
            mime_types: other.mime_types.or(self.mime_types.clone()),
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> ConfigBuilder {
        ConfigBuilder {
            html: Some(PathBuf::from("index.html")),
            css: None,
            js: None,
            unsafe_inline: Some(false),
            host: Some(String::from("0.0.0.0")),
            port: Some(3333),
            static_path: None,
            static_content: Some(PathBuf::from("static")),
            mime_types: Some(PathBuf::from("/etc/mime.types")),
        }
    }
}
