pub mod external;
pub mod native;
pub mod progress;

use crate::cli::Args;
use crate::error::AppError;
use external::{ExternalDownloader, ExternalTool};
use native::NativeDownloader;

pub enum DownloaderKind {
    Native(NativeDownloader),
    External(ExternalDownloader),
}

impl DownloaderKind {
    pub fn from_args(args: &Args) -> Result<Self, AppError> {
        if let Some(ref wget_bin) = args.wget {
            return Ok(Self::External(ExternalDownloader::new(
                ExternalTool::Wget,
                Some(wget_bin.clone()),
            )));
        }

        if let Some(ref curl_bin) = args.curl {
            return Ok(Self::External(ExternalDownloader::new(
                ExternalTool::Curl,
                Some(curl_bin.clone()),
            )));
        }

        Ok(Self::Native(NativeDownloader::new(args.fragments)?))
    }

    pub async fn download(&self, url: &str, filename: &str) -> Result<bool, AppError> {
        match self {
            Self::Native(d) => d.download(url, filename).await,
            Self::External(d) => d.download(url, filename),
        }
    }
}
