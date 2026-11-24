use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("CLI argument error: {0}")]
    Cli(#[from] clap::Error),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse page HTML: {0}")]
    Parse(String),

    #[error("Obfuscation decode failed: {0}")]
    Decode(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("External downloader '{0}' exited with non-zero status")]
    ExternalDownloader(String),
}
