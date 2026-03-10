# Ferrox

Ferrox is a small web server written in Rust from first principles. The goal of the project is to explore how a simple HTTP server works internally without hiding the fundamentals behind a large framework.

It is currently focused on one job: serving static files from a `www/` directory with a minimal HTTP stack implemented inside the project.

## Why this project exists

Ferrox is a learning-oriented server project built to better understand:

- TCP-based request handling in Rust
- Manual HTTP request parsing and response writing
- Static file serving
- MIME type detection
- Safe path resolution for files on disk
- Clean fallback error pages

This is not trying to compete with production web servers. It is a compact, readable codebase for experimentation and iteration.

## Current features

- Serves files from the `www/` directory
- Resolves directories to `index.html`
- Detects content types with `mime_guess`
- Returns custom HTML error pages for `403`, `404`, and `500` cases
- Rejects directory traversal outside the serving root
- Handles connections concurrently using one thread per incoming stream
- Includes a minimal default homepage so the server is usable immediately

## How it works

At a high level, Ferrox:

1. Binds a TCP listener
2. Accepts incoming connections
3. Reads the raw request into a buffer
4. Parses the request line into method, path, and HTTP version
5. Maps the requested path into the `www/` directory
6. Serves the file or returns an error page
7. Writes a full HTTP response back to the client

The code is intentionally split into small modules for server logic, request/response types, error rendering, and static file handling.

## Project structure

```text
src/
  main.rs              Entry point
  server.rs            TCP listener and request handling
  handlers/
    static_files.rs    Static file resolution and delivery
  http/
    request.rs         Minimal HTTP request parsing
    response.rs        HTTP response formatting and writing
    error.rs           Error page rendering
templates/
  error.html           Shared HTML template for error pages
www/
  index.html           Default site content
```

## Running locally

```bash
cargo run
```

Then open:

```text
http://127.0.0.1/
```

Important note: the server currently binds to `0.0.0.0:80`. On many systems, binding to port `80` requires elevated privileges. If that is inconvenient during development, changing the port in `src/main.rs` to something like `8080` is the simplest option.

## Current limitations

Ferrox is intentionally minimal right now. A few practical limitations of the current version:

- The request buffer is fixed and small
- Body is not parsed yet(no use for now)
- There is no routing layer beyond static file serving
- There is no keep-alive support
- Error handling is intentionally simple
- The concurrency model is thread-per-connection
- It should be treated as an experimental project, not a hardened production server

## Future direction

Natural next steps for the project could include:

- TOML/YAML/JSON configuration
- Better request parsing
- Logging improvements
- Smarter error handling
- TLS Support
- Performance improvements beyond thread-per-connection
- Tests and benchmarks
- Become better than at least **CERN httpd**

## Important note

Technically, if your only goal is to serve static files somewhere other than **World Wide Web**, you could already use **Ferrox** for that.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).
