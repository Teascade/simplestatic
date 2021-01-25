use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Reach new heights.
#[argh(description = "Simple http server for serving a single simple html file ie. as a maintenance or 404 page.")]
pub struct MainArgs {
    #[argh(option, short = 'h', description = "path to the served html file")]
    html_path: Option<PathBuf>,

    #[argh(option, short = 'c', description = "path to the embedded css file, or folder containing the css files")]
    css_path: Option<PathBuf>,

    #[argh(option, short = 'j', description = "path to the embedded js file, or folder containing the js files")]
    js_path: Option<PathBuf>,
}