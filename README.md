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
- Add the capability of serving static content

## License

Simple Static is licensed under the AGPLv3 license.