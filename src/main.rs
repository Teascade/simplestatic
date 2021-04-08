use args::MainArgs;
use config::ConfigBuilder;
use files::Mimetypes;
use std::env;
use std::net::IpAddr;
use std::path::PathBuf;
use template::Template;
use warp::http::header::{HeaderMap, HeaderValue};
use warp::Filter;

mod args;
mod config;
mod errors;
mod files;
mod template;

#[tokio::main]
async fn main() {
    let args: MainArgs = argh::from_env();
    let env_path = env::var("SSTATIC_CONFIG_PATH").ok().map(PathBuf::from);
    let config_path = args.config_path.clone().or(env_path);

    let config = match ConfigBuilder::default()
        .or_from_env()
        .or_from_file(config_path)
    {
        Ok(config) => match config.or_from_cmd(args).build() {
            Ok(config) => config,
            Err(_) => panic!("Failed to build config, something is horribly wrong!"),
        },
        Err(e) => panic!(e),
    };

    if config.static_path.clone().contains('/') {
        panic!("Unsupported feature: static_path should not contain \"/\"");
    }

    let mimetypes = match Mimetypes::try_fetch(&config.mime_types) {
        Ok(m) => m,
        Err(_) => Mimetypes::default(),
    };

    let (template, mut js_hashes, mut css_hashes) = match files::get_files(&config) {
        Ok((html, css, js)) => match Template::new(html, css, js) {
            Ok(template) => template,
            Err(e) => panic!("Error: {}", e),
        },
        Err(e) => panic!("Error: {}", e),
    };

    if js_hashes.is_empty() {
        js_hashes.push("'none'".to_owned());
    }
    if css_hashes.is_empty() {
        css_hashes.push("'none'".to_owned());
    }

    let js_hashes = js_hashes.join(" ");
    let css_hashes = css_hashes.join(" ");

    let csp = if config.unsafe_inline {
        format!("default-src 'self'; script-src 'unsafe-inline'; style-src 'unsafe-inline';")
    } else {
        if template.unsafe_inline {
            eprint!("\u{001b}[3;91m");
            eprintln!("Some newlines in script or css tags were not minified correctly. Due to Content-Security-Policy, the site may not work correctly.");
            eprintln!("Use --unsafe-inline -flag to use 'unsafe-inline' Content-Security-Policy to ignore this error message.");
            eprint!("\u{001b}[0m");
        }
        format!(
            "default-src 'self'; script-src {}; style-src {};",
            js_hashes, css_hashes
        )
    };

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("text/html"));
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_str(&csp).unwrap(),
    );

    let host: IpAddr = if let Ok(ip) = config.host.clone().parse() {
        ip
    } else {
        panic!("Unable to parse host address");
    };
    let port = config.port.into();
    println!("Serving maintenance page on {}:{}", host, port);

    let maintenance = warp::any()
        .and(warp::header("Host"))
        .and(warp::header("User-Agent"))
        .map(move |host: String, ua: String| template.render(host, ua))
        .map(|reply| warp::reply::with_status(reply, warp::http::StatusCode::SERVICE_UNAVAILABLE))
        .with(warp::reply::with::headers(headers));

    if let Some(static_content) = config.static_content.clone() {
        let content_path = static_content.canonicalize().unwrap();
        let static_serve = warp::path(config.static_path)
            .and(warp::path::param::<String>())
            .and(warp::path::end())
            .map(move |path: String| files::serve_file(&mimetypes, &content_path, path));

        warp::serve(static_serve.or(maintenance))
            .run((host, port))
            .await;
    } else {
        warp::serve(maintenance).run((host, port)).await;
    }
}
