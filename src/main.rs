#![forbid(unsafe_code)]

use clap::Parser;
use einthusan_dl::cli::Args;
use einthusan_dl::run_app;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    if let Err(e) = run_app(args).await {
        eprintln!("Error: {e}");
    }
}
