# Simple Static

![Crates.io](https://img.shields.io/crates/v/simplestatic)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/transyhdistys/simplestatic/rust)
![GitHub](https://img.shields.io/github/license/transyhdistys/simplestatic)

Simple Static is a simple http web server that serves one simple http file and other possible resources.
The reason for Simple Static's existance is to provide a back-end webserver for simple cases such as a maintenance
page or a 404 page for cases where the proxy does not serve static sites itself.

Simple Static is able to embed javascript and css files from given paths to the html file and also handles 
Content-Security-Policy so that any inline styles or scripts are included in the CSP for maximum security.

Simple Static is also capable of serving other static content, but it's purpose is strictly to serve content
like images for the main html file.

# Deployment

## Install from cargo 

Run `cargo install simplestatic`

And then just run `simplestatic`. Use `--help` for help.

## Docker

Simplestatic can be pulled from
`ghcr.io/transyhdistys/simplestatic:latest`

So running it can be simple as
`docker run -dp 3333:3333 ghcr.io/transyhdistys/simplestatic:latest`

Or for docker-compose
```yaml
simplestatic:
    image: ghcr.io/transyhdistys/simplestatic:latest
    environment:
      - SSTATIC_PORT: 3333 # See configuring for more
    ports:
      - 3333:3333
```

## Build manually

Simply install Rust and run `cargo run`. It should not have other dependencies, and at least Rust 1.48 is new enough.

# Performance

On my personal computer when testing with [wrk](https://github.com/wg/wrk),
Simple Static is able to serve about 150,000 requests per second, which should be plenty for anything.

100 MB of RAM is probably plenty for all kinds of maintenance websites (unless
if you're using big javascript libraries, in which case you may need more).

As for CPU I can't quite tell. A toaster should be able to run this, but with
enough traffic it will probably eat all of the CPU you give it.

## Configuring Simple Static

Configuration can be done in three different ways. 

| Environment Variables    | Command-Line Arguments  | config.toml     | Description
|--------------------------|-------------------------|-----------------|------------------------------
| `SSTATIC_HTML_PATH`      | `--html`                | `html`          | Path to the single html file.
| `SSTATIC_JS_PATH`        | `--js`                  | `js`            | Path to the javascript file to embed, or folder containing the javascript files to embed.
| `SSTATIC_CSS_PATH`       | `--css`                 | `css`           | Path to the css file to embed, or folder containing the css files to embed.
| `SSTATIC_UNSAFE_INLINE`  | `--unsafe-inline`       | `unsafe_inline` | Allow usage of unsafe-inline CSP policy.
| `SSTATIC_PORT`           | `--port`                | `port`          | Port to bind to.
| `SSTATIC_HOST`           | `--host`                | `host`          | Host address to bind to.
| `SSTATIC_STATIC_PATH`    | `--static-path`         | `static_path`   | Path that will serve the static content
| `SSTATIC_STATIC_CONTENT` | `--static-content`      | `static_content`| Path of the content that will be served
| `SSTATIC_MIME_TYPES`     | `--mime-types`          | `mime_types`    | Path to file containing mime types.
| `SSTATIC_CONFIG_PATH`    | `--config-path`         | No option       | Path to an optional config.toml file.

The order in which these are prioritized from the first priority to last are
1. Command-Line Arguments
2. config.toml
3. Environment Variables

An example toml configuration can be found at [`config.sample.toml`](config.sample.toml). The default config file is `config.toml` at working directory
and `config.sample.toml` is configured to use the default configurations.

## Templating

Simple Static supports a small bit of templating in order to customize the webpage for each request.

1. At the start of the program it looks for entries of `{{ js }}` and `{{ css }}` in the html file. This is where it embeds any external 
javascript and css files it finds respectively.
2. Every time when the page is rendered, a few additional templates are filled. Refer to the table below:

| What                  | Turns into          |
|-----------------------|---------------------|
| `{{ Host }}`          | Host -header        |
| `{{ User-Agent }}`    | User-Agent -header  |

## License

Simple Static is licensed under the [AGPLv3](./LICENSE) license.
