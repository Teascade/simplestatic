# Simple Static

Simple Static is a simple http web server that serves one simple http file and other possible resources.
The reason for Simple Static's existance is to provide a back-end webserver for simple cases such as a maintenance
page or a 404 page for cases where the proxy does not serve static sites itself.

Simple Static is able to embed javascript and css files from given paths to the html file and also handles 
Content-Security-Policy so that any inline styles or scripts are included in the CSP for maximum security.

Simple Static is also capable of serving other static content, but it's purpose is strictly to serve content
like images for the main html file.

Todos:
- Add a configuration file for anyone who does not wish to use command-line arguments
- Add environment variable reading for additional configuration possibilities.
- Add the capability of serving static content

## Configuring Simple Static

Configuration can be done in three different ways. 

| Environment Variables  | Command-Line Arguments  | config.toml     | Description
|------------------------|-------------------------|-----------------|------------------------------
| `STATIC_HTML_PATH`     | `--html`                | `html-path`     | Path to the single html file.
| `STATIC_JS_PATH`       | `--js`                  | `js-path`       | Path to the javascript file to embed, or folder containing the javascript files to embed.
| `STATIC_CSS_PATH`      | `--css`                 | `css-path`      | Path to the css file to embed, or folder containing the css files to embed.
| `STATIC_UNSAFE_INLINE` | `--unsafe-inline`       | `unsafe-inline` | Allow usage of unsafe-inline CSP policy.

The order in which these are prioritized from the first priority to last are
- Command-Line Arguments
- Environment Variables
- config.toml

## License

Simple Static is licensed under the AGPLv3 license.