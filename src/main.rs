use std::fs;
use std::path::PathBuf;

use warp::Filter;

mod args;

use args::MainArgs;

#[tokio::main]
async fn main() {

    let args: MainArgs = argh::from_env();

    dbg!(args);

    let path = fs::canonicalize(PathBuf::from("otus.html")).unwrap();

    let file = match fs::read(path) {
        Ok(file) => file,
        Err(e) => panic!("Could not read file: {}", e),
    };

    let hello = warp::any()
        .map(|| format!("Hello, Someone!"));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}