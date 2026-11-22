use futures_util::{future::join_all, StreamExt};
use reqwest::header::{CONTENT_LENGTH, CONTENT_RANGE, RANGE};
use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{error, info, warn};

use crate::downloader::progress::DownloadProgress;
use crate::error::AppError;

const MAX_ATTEMPTS: u32 = 5;
const FRAGMENT_COUNT: usize = 4;

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

        let mut attempts = 0;
        let mut error_msg = String::new();

        while attempts < MAX_ATTEMPTS {
            match self.download_once(url, filename).await {
                Ok(true) => return Ok(true),
                Ok(false) => {
                    error_msg = "server returned an unsuccessful HTTP status".to_string();
                }
                Err(e) => {
                    error_msg = e.to_string();
                }
            }

            attempts += 1;
            if attempts < MAX_ATTEMPTS {
                let wait_interval = 2u64.pow(attempts);
                println!(
                    "Error downloading, will retry in {} seconds ...",
                    wait_interval
                );
                tokio::time::sleep(Duration::from_secs(wait_interval)).await;
            }
        }

        warn!("Skipping, can't download file ...");
        error!("{}", error_msg);
        Ok(false)
    }

    async fn download_once(&self, url: &str, filename: &str) -> Result<bool, AppError> {
        let response = self
            .client
            .get(url)
            .header(RANGE, "bytes=0-0")
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => {
                self.download_full_response(response, filename).await?;
                Ok(true)
            }
            reqwest::StatusCode::PARTIAL_CONTENT => {
                let total = response
                    .headers()
                    .get(CONTENT_RANGE)
                    .and_then(|value| value.to_str().ok())
                    .and_then(parse_content_range_total)
                    .ok_or_else(|| {
                        AppError::Parse("range response did not include a total size".to_string())
                    })?;

                self.download_fragments(url, filename, total).await?;
                Ok(true)
            }
            status => {
                warn!("Download request returned HTTP {status}");
                Ok(false)
            }
        }
    }

    async fn download_full_response(
        &self,
        response: reqwest::Response,
        filename: &str,
    ) -> Result<(), AppError> {
        let content_len = response
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse::<u64>().ok());
        let progress = Arc::new(Mutex::new(DownloadProgress::new(content_len)));
        progress.lock().expect("progress mutex poisoned").start();

        let file_res = File::create(Path::new(filename));
        let raw_file = match file_res {
            Ok(f) => f,
            Err(e) => return Err(AppError::Io(e)),
        };
        let mut file = std::io::BufWriter::with_capacity(1024 * 1024, raw_file);
        let mut stream = response.bytes_stream();
        let ctrl_c = tokio::signal::ctrl_c();
        tokio::pin!(ctrl_c);

        loop {
            tokio::select! {
                biased;
                _ = &mut ctrl_c => {
                    info!("Keyboard Interrupt -- Removing partial file: {}", filename);
                    let _ = std::fs::remove_file(filename);
                    std::process::exit(130);
                }
                chunk_opt = stream.next() => {
                    match chunk_opt {
                        Some(Ok(chunk)) => {
                            file.write_all(&chunk)?;
                            progress.lock().expect("progress mutex poisoned").read(chunk.len() as u64);
                        }
                        Some(Err(e)) => {
                            let _ = std::fs::remove_file(filename);
                            return Err(AppError::Http(e));
                        }
                        None => {
                            file.flush()?;
                            progress.lock().expect("progress mutex poisoned").stop();
                            return Ok(());
                        }
                    }
                }
            }
        }
    }

    async fn download_fragments(
        &self,
        url: &str,
        filename: &str,
        total: u64,
    ) -> Result<(), AppError> {
        let ranges = split_ranges(total);
        let file = File::create(Path::new(filename))?;
        file.set_len(total)?;
        drop(file);

        let fragment_totals = ranges.iter().map(|(start, end)| end - start + 1).collect();
        let progress = Arc::new(Mutex::new(DownloadProgress::new_fragmented(
            fragment_totals,
        )));
        progress.lock().expect("progress mutex poisoned").start();

        // shortcut: fixed four connections; add a user setting if server throttling requires tuning
        let requests = ranges.into_iter().enumerate().map(|(index, (start, end))| {
            self.download_fragment(url, filename, index, start, end, Arc::clone(&progress))
        });
        let results = join_all(requests).await;

        if let Some(error) = results.into_iter().find_map(Result::err) {
            let _ = std::fs::remove_file(filename);
            return Err(error);
        }

        progress.lock().expect("progress mutex poisoned").stop();
        Ok(())
    }

    async fn download_fragment(
        &self,
        url: &str,
        filename: &str,
        index: usize,
        start: u64,
        end: u64,
        progress: Arc<Mutex<DownloadProgress>>,
    ) -> Result<(), AppError> {
        let response = self
            .client
            .get(url)
            .header(RANGE, format!("bytes={start}-{end}"))
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
            return Err(AppError::Parse(format!(
                "fragment request returned HTTP {}",
                response.status()
            )));
        }

        let mut file = OpenOptions::new().write(true).open(Path::new(filename))?;
        file.seek(SeekFrom::Start(start))?;
        let mut stream = response.bytes_stream();
        let expected = end - start + 1;
        let mut received = 0u64;
        let ctrl_c = tokio::signal::ctrl_c();
        tokio::pin!(ctrl_c);

        loop {
            tokio::select! {
                biased;
                _ = &mut ctrl_c => {
                    info!("Keyboard Interrupt -- Removing partial file: {}", filename);
                    let _ = std::fs::remove_file(filename);
                    std::process::exit(130);
                }
                chunk_opt = stream.next() => {
                    match chunk_opt {
                        Some(Ok(chunk)) => {
                            let chunk_len = chunk.len() as u64;
                            if received + chunk_len > expected {
                                return Err(AppError::Parse(format!(
                                    "fragment exceeded requested range {start}-{end}"
                                )));
                            }
                            file.write_all(&chunk)?;
                            received += chunk_len;
                            progress
                                .lock()
                                .expect("progress mutex poisoned")
                                .read_fragment(index, chunk_len);
                        }
                        Some(Err(e)) => return Err(AppError::Http(e)),
                        None => {
                            if received != expected {
                                return Err(AppError::Parse(format!(
                                    "fragment ended at {received} bytes, expected {expected}"
                                )));
                            }
                            return Ok(());
                        }
                    }
                }
            }
        }
    }
}

fn split_ranges(total: u64) -> Vec<(u64, u64)> {
    if total == 0 {
        return Vec::new();
    }

    let count = if total < FRAGMENT_COUNT as u64 {
        total as usize
    } else {
        FRAGMENT_COUNT
    };
    let base_size = total / count as u64;
    let remainder = total % count as u64;
    let mut start = 0;
    let mut ranges = Vec::with_capacity(count);

    for index in 0..count {
        let size = base_size + u64::from((index as u64) < remainder);
        let end = start + size - 1;
        ranges.push((start, end));
        start = end + 1;
    }

    ranges
}

fn parse_content_range_total(value: &str) -> Option<u64> {
    let total = value.split_once('/')?.1;
    (total != "*").then(|| total.parse().ok()).flatten()
}

#[cfg(test)]
mod tests {
    use super::{parse_content_range_total, split_ranges};

    #[test]
    fn split_ranges_cover_file_without_gaps() {
        let ranges = split_ranges(10);

        assert_eq!(ranges, vec![(0, 2), (3, 5), (6, 7), (8, 9)]);
        assert_eq!(ranges.first().unwrap().0, 0);
        assert_eq!(ranges.last().unwrap().1, 9);
        assert!(ranges.windows(2).all(|pair| pair[0].1 + 1 == pair[1].0));
    }

    #[test]
    fn split_ranges_use_one_fragment_for_tiny_files() {
        assert_eq!(split_ranges(2), vec![(0, 0), (1, 1)]);
    }

    #[test]
    fn content_range_total_requires_known_total() {
        assert_eq!(parse_content_range_total("bytes 0-0/10"), Some(10));
        assert_eq!(parse_content_range_total("bytes 0-0/*"), None);
    }
}
