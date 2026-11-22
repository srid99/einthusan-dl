# Einthusan Downloader

A fast, standalone movie downloader for [Einthusan][1], rewritten in **Rust** 🦀 — single binary, zero external runtime dependencies, no Python required.

[Einthusan][1] is a video publisher providing movies and music videos on-demand. `einthusan-dl` allows downloading movies from Einthusan site directly from your command line.

## Features

- **Fast & Lightweight**: Single ~4.4MB native binary, instant startup (~270ms vs >2.4s previously).
- **Zero Runtime Dependencies**: Pure static Rust binary using modern async I/O.
- **Multiple Download Engines**: Built-in native downloader with retry backoff and progress bar, plus optional external `--wget` and `--curl` support.
- **Batch Processing**: Download multiple movie URLs sequentially in one run.
- **Movie Plot Output**: Use `--plot` to print the plot from the movie page.
- **Version Information**: Use `--version` to print the release and native download default.
- **Resilient**: Graceful handling of network disconnects and clean partial-file removal on `Ctrl-C`.

## Installation & Build

Requires a standard Rust toolchain (Rust 1.75+):

```bash
# Clone the repository
git clone https://github.com/sunilsankar/einthusan-dl.git
cd einthusan-dl

# Build release executable
cargo build --release

# The compiled binary will be at:
# target/release/einthusan-dl (Linux/macOS)
# target/release/einthusan-dl.exe (Windows)

# Optionally install to your cargo bin path (~/.cargo/bin)
cargo install --path .
```

## Usage

```bash
# General usage
einthusan-dl "<movie_url1>"

# Multiple movies in batch
einthusan-dl "<movie_url1>" "<movie_url2>" "<movie_url3>"

# Specify download directory
einthusan-dl --path=/path/to/downloads "<movie_url1>"

# Specify external downloader (wget or curl)
einthusan-dl "<movie_url1>" --wget
einthusan-dl "<movie_url1>" --curl

# Overwrite existing files
einthusan-dl "<movie_url1>" -o

# Print the movie plot without downloading the movie
einthusan-dl "<movie_url1>" --plot

# Specify log file
einthusan-dl --log=/path/to/output.log "<movie_url1>"

# Display help and options
einthusan-dl --help

# Print the release and native parallel-download default
einthusan-dl --version
```

> **Note:** Always enclose the URL in double quotes (`"`) to prevent shell interpretation of query parameters (like `?lang=tamil`).

### Finding Movie URLs

1. Open [Einthusan](https://einthusan.tv) in your browser.
2. Navigate to the movie you wish to download.
3. Copy the URL from the browser address bar. It should follow this format:
   ```
   https://einthusan.tv/movie/watch/{MOVIE-ID}/
   ```

## Release Notes

### v2.0.1

- Added `--plot` to print a movie's plot without downloading it.
- Added `--version` to print the release and the default four-way native parallelism.
- Native downloads use four concurrent byte ranges when supported, with per-fragment retries.
- Servers without range support use the single-stream fallback.

## Disclaimer

As per the [`Terms of Service`][4] from Einthusan:

> "You may not either directly or through the use of any device,
software, internet site, web-based service, or other means copy, download, stream capture, reproduce, duplicate,
archive, distribute, upload, publish, modify, translate, broadcast, perform, display, sell, transmit or retransmit
the Content unless expressly permitted by Einthusan in writing."

Please use this utility responsibly and strictly at your own risk.

[1]: https://www.einthusan.tv
[4]: https://einthusan.tv/terms/
