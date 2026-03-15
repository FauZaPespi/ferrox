# Ferrox

Ferrox is a fast and lightweight web server written in Rust from first principles. The goal of the project is to explore how an HTTP server works internally without hiding the fundamentals behind a large framework, while still achieving high performance.

It is currently focused on one job: serving static files from a `www/` directory with a custom HTTP stack implemented inside the project.

## Why this project exists

Ferrox is a learning-oriented server project built to better understand:

- Asynchronous TCP-based request handling in Rust (using Tokio)
- Manual HTTP request parsing and response writing
- Static file serving and MIME type detection
- Safe path resolution for files on disk to prevent path traversal
- Clean fallback error pages

This codebase is designed to be compact, readable, and an excellent foundation for experimentation and iteration.

## Current features

- Serves files from the `www/` directory
- Resolves directories to `index.html` or generates directory indexes
- Detects content types with `mime_guess`
- Returns custom HTML error pages for `400`, `403`, `404`, and `500` cases
- Rejects directory traversal outside the serving root using canonicalization
- **High Performance:** Handles thousands of concurrent connections using Tokio's lightweight asynchronous tasks (Event-Driven architecture).
- **Robust Parsing:** Safely parses HTTP requests using dynamic buffers, protecting against malformed data without panicking.
- **Security:** Includes connection timeouts to mitigate Slowloris attacks.

## How it works

At a high level, Ferrox:

1. Binds an asynchronous TCP listener
2. Accepts incoming connections and spawns a lightweight Tokio task for each
3. Safely reads the raw request into a dynamic buffer until the `\r\n\r\n` boundary
4. Parses the request line into method, path, and HTTP version without allocations where possible
5. Maps the requested path into the `www/` directory safely
6. Serves the file (using async I/O) or returns an error page
7. Writes a full HTTP response back to the client

The code is intentionally split into small modules for server logic, request/response types, error rendering, and static file handling.

## Project structure

```text
src/
  main.rs              Entry point
  server.rs            Async TCP listener and request handling
  handlers/
    static_files.rs    Static file resolution and async delivery
  http/
    request.rs         Minimal, panic-free HTTP request parsing
    response.rs        HTTP response formatting and writing
  utils/
    logger.rs          Request and error logging
    templates.rs       Basic HTML template rendering
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

- Body is not parsed yet (no use for now, only GET/HEAD are practically supported)
- There is no routing layer beyond static file serving
- There is no keep-alive support (`Connection: close` is hardcoded)
- It should be treated as an experimental project, not a hardened production server (yet)

## Future direction

Natural next steps for the project could include:

- TOML/YAML/JSON configuration parsing
- TLS Support (HTTPS) via `tokio-rustls`
- Zero-copy request parsing for even lower memory footprint
- Keep-alive support for persistent connections
- Tests and benchmarks
- Become better than at least **CERN httpd**

## Important note

Technically, if your only goal is to serve static files somewhere other than the **World Wide Web**, you could already use **Ferrox** for that. It is fast, secure against basic traversal, and very lightweight.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).