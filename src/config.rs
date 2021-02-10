use crate::args::MainArgs;
use std::env;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub html: PathBuf,
    pub css: Option<PathBuf>,
    pub js: Option<PathBuf>,
    pub unsafe_inline: bool,
    pub port: u16,
}

pub struct ConfigBuilder {
    html: Option<PathBuf>,
    css: Option<PathBuf>,
    js: Option<PathBuf>,
    unsafe_inline: Option<bool>,
    port: Option<u16>,
}

impl ConfigBuilder {
    pub fn build(&self) -> Result<Config, ()> {
        if self.html.is_none() || self.unsafe_inline.is_none() || self.port.is_none() {
            Err(())
        } else {
            Ok(Config {
                html: self.html.clone().unwrap(),
                css: self.css.clone(),
                js: self.js.clone(),
                unsafe_inline: self.unsafe_inline.unwrap(),
                port: self.port.unwrap(),
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
            port: args.port,
        })
    }

    pub fn or_from_env(&self) -> ConfigBuilder {
        self.or_rather(ConfigBuilder {
            html: env::var("SSTATIC_HTML_PATH").ok().map(PathBuf::from),
            css: env::var("SSTATIC_JS_PATH").ok().map(PathBuf::from),
            js: env::var("SSTATIC_CSS_PATH").ok().map(PathBuf::from),
            unsafe_inline: env::var("SSTATIC_UNSAFE_INLINE").ok().map(|_| true),
            port: env::var("SSTATIC_PORT")
                .ok()
                .map(|x| x.parse::<u16>().unwrap_or(3333)),
        })
    }

    pub fn or_rather(&self, other: ConfigBuilder) -> ConfigBuilder {
        ConfigBuilder {
            html: other.html.or(self.html.clone()),
            css: other.css.or(self.css.clone()),
            js: other.js.or(self.js.clone()),
            unsafe_inline: other.unsafe_inline.or(self.unsafe_inline),
            port: other.port.or(self.port),
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
            port: Some(3333),
        }
    }
}
