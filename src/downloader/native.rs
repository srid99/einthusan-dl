use futures_util::StreamExt;
use reqwest::header::CONTENT_LENGTH;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::time::Duration;
use tracing::{error, info, warn};

use crate::downloader::progress::DownloadProgress;
use crate::error::AppError;

pub struct NativeDownloader {
    client: reqwest::Client,
}

impl NativeDownloader {
    pub fn new() -> Result<Self, AppError> {
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(15))
            .read_timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self { client })
    }

    pub async fn download(&self, url: &str, filename: &str) -> Result<bool, AppError> {
        info!("Downloading {} -> {}", url, filename);

        let mut attempts_count = 0;
        let mut error_msg = String::new();

        while attempts_count < 5 {
            let res = self.client.get(url).send().await;

            let resp = match res {
                Ok(r) if r.status() == reqwest::StatusCode::OK => r,
                Ok(r) => {
                    warn!("Probably the file is missing from the AWS repository... waiting.");
                    error_msg = format!("HTTP Error {}", r.status());
                    let wait_interval = 2u64.pow((attempts_count + 1) as u32);
                    println!(
                        "Error downloading, will retry in {} seconds ...",
                        wait_interval
                    );
                    tokio::time::sleep(Duration::from_secs(wait_interval)).await;
                    attempts_count += 1;
                    continue;
                }
                Err(e) => {
                    warn!("Probably the file is missing from the AWS repository... waiting.");
                    error_msg = format!("Request error: {e}");
                    let wait_interval = 2u64.pow((attempts_count + 1) as u32);
                    println!(
                        "Error downloading, will retry in {} seconds ...",
                        wait_interval
                    );
                    tokio::time::sleep(Duration::from_secs(wait_interval)).await;
                    attempts_count += 1;
                    continue;
                }
            };

            let content_len = resp
                .headers()
                .get(CONTENT_LENGTH)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok());

            let mut progress = DownloadProgress::new(content_len);
            progress.start();

            let file_res = File::create(Path::new(filename));
            let raw_file = match file_res {
                Ok(f) => f,
                Err(e) => return Err(AppError::Io(e)),
            };
            let mut file = BufWriter::with_capacity(1024 * 1024, raw_file);

            let mut stream = resp.bytes_stream();

            // Stream chunks directly to disk
            loop {
                tokio::select! {
                    biased;
                    _ = tokio::signal::ctrl_c() => {
                        info!("Keyboard Interrupt -- Removing partial file: {}", filename);
                        let _ = std::fs::remove_file(filename);
                        std::process::exit(130);
                    }
                    chunk_opt = stream.next() => {
                        match chunk_opt {
                            Some(Ok(chunk)) => {
                                if let Err(e) = file.write_all(&chunk) {
                                    let _ = std::fs::remove_file(filename);
                                    return Err(AppError::Io(e));
                                }
                                progress.read(chunk.len() as u64);
                            }
                            Some(Err(e)) => {
                                warn!("Stream interrupted: {e}");
                                let _ = std::fs::remove_file(filename);
                                attempts_count += 1;
                                break;
                            }
                            None => {
                                if let Err(e) = file.flush() {
                                    let _ = std::fs::remove_file(filename);
                                    return Err(AppError::Io(e));
                                }
                                progress.stop();
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        warn!("Skipping, can't download file ...");
        error!("{}", error_msg);
        Ok(false)
    }
}
