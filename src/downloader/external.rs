use std::process::Command;
use tracing::{debug, info};

use crate::error::AppError;

pub enum ExternalTool {
    Wget,
    Curl,
}

pub struct ExternalDownloader {
    tool: ExternalTool,
    bin: String,
}

impl ExternalDownloader {
    pub fn new(tool: ExternalTool, custom_bin: Option<String>) -> Self {
        let default_bin = match tool {
            ExternalTool::Wget => "wget",
            ExternalTool::Curl => "curl",
        };
        let bin = custom_bin.unwrap_or_else(|| default_bin.to_string());
        Self { tool, bin }
    }

    pub fn download(&self, url: &str, filename: &str) -> Result<bool, AppError> {
        let args: Vec<&str> = match self.tool {
            ExternalTool::Wget => vec![
                url,
                "-O",
                filename,
                "--no-cookies",
                "--no-check-certificate",
            ],
            ExternalTool::Curl => vec![url, "-k", "-#", "-L", "-o", filename],
        };

        debug!("Executing {}: {} {:?}", self.bin, self.bin, args);

        let mut child = Command::new(&self.bin).args(&args).spawn().map_err(|e| {
            AppError::Io(std::io::Error::new(
                e.kind(),
                format!("{e}. Are you sure that '{}' is the right bin?", self.bin),
            ))
        })?;

        let status = child.wait().map_err(AppError::Io)?;
        if !status.success() {
            info!("External downloader exited with non-zero code");
            return Ok(false);
        }

        Ok(true)
    }
}
