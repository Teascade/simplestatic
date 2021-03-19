use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, Debug, Clone)]
/// Reach new heights.
#[argh(
    description = "Simple http server for serving a single simple html file ie. as a maintenance or 404 page."
)]
pub struct MainArgs {
    #[argh(option, short = 'h', description = "path to the served html file")]
    pub html: Option<PathBuf>,

    #[argh(
        option,
        short = 'c',
        description = "path to the embedded css file, or folder containing the css files"
    )]
    pub css: Option<PathBuf>,

    #[argh(
        option,
        short = 'j',
        description = "path to the embedded js file, or folder containing the js files"
    )]
    pub js: Option<PathBuf>,

    #[argh(switch, description = "allow unsafe_inline content-security-policy")]
    pub unsafe_inline: bool,

    #[argh(
        option,
        short = 'i',
        description = "set host that the server is served on. defaults to 0.0.0.0"
    )]
    pub host: Option<String>,

    #[argh(
        option,
        short = 'p',
        description = "set port that will be exposed and served on. defaults to 3333"
    )]
    pub port: Option<u16>,

    #[argh(
        option,
        short = 's',
        description = "set path where to serve static content. \"static\" will serve at /static/files"
    )]
    pub static_path: Option<String>,

    #[argh(
        option,
        short = 'f',
        description = "path to the served static file, or folder containing served static files. defaults to \"static\""
    )]
    pub static_content: Option<PathBuf>,

    #[argh(
        option,
        short = 'm',
        description = "path to mimetypes. defaults to /etc/mime.types, uses default mimetypes on error."
    )]
    pub mime_types: Option<PathBuf>,
}
