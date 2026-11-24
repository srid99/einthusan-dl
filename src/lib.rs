#![forbid(unsafe_code)]

pub mod cli;
pub mod client;
pub mod downloader;
pub mod error;
pub mod logging;
pub mod scraper;

use std::fs;
use std::time::Instant;
use tracing::{debug, error, info};

use crate::cli::Args;
use crate::client::EinthusanClient;
use crate::downloader::DownloaderKind;
use crate::error::AppError;
use crate::scraper::slugify_filename;

pub async fn run_app(args: Args) -> Result<(), AppError> {
    logging::init_logging(args.logfile.as_deref(), args.debug)?;

    let client = EinthusanClient::new()?;

    if args.plot {
        for movie_page_url in &args.urls {
            if let Err(e) = print_movie_plot(&client, movie_page_url).await {
                error!("Failed to fetch plot for {}: {}", movie_page_url, e);
            }
        }
        return Ok(());
    }

    let downloader = DownloaderKind::from_args(&args)?;

    for movie_page_url in &args.urls {
        if let Err(e) = process_single_movie(&client, &downloader, &args, movie_page_url).await {
            error!("Failed to process movie {}: {}", movie_page_url, e);
        }
    }

    Ok(())
}

async fn process_single_movie(
    client: &EinthusanClient,
    downloader: &DownloaderKind,
    args: &Args,
    movie_page_url: &str,
) -> Result<(), AppError> {
    info!("Downloading movie from page : {}", movie_page_url);
    let (meta, links) = client.resolve_movie_info(movie_page_url).await?;

    let movie_name = meta.movie_name;
    let movie_url = links.mp4_link;

    // shortcut: truncate-write per movie; with multiple URLs the last name wins, upgrade to append/one file per movie if needed
    if let Some(path) = &args.output_movie_name {
        fs::write(path, &movie_name)?;
    }

    debug!(
        "Start downloading movie [{}] from url {}",
        movie_name, movie_url
    );

    let slugified_name = slugify_filename(&movie_name);
    let dest_dir = if args.path.is_empty() {
        std::path::PathBuf::from(&slugified_name)
    } else {
        std::path::Path::new(&args.path).join(&slugified_name)
    };

    if !dest_dir.exists() {
        debug!(
            "Destination path {} not exists. Let's create one.",
            dest_dir.display()
        );
        fs::create_dir_all(&dest_dir)?;
    }

    let filename = dest_dir.join(format!("{}.mp4", slugified_name));
    let filename_str = filename.to_string_lossy().to_string();

    let file_exists = filename.exists();

    if args.overwrite || !file_exists {
        if file_exists {
            info!("File already exists and will be overwritten since option \"--overwrite\" is enabled.");
        }

        if !args.skip_download {
            info!("Downloading: {}", movie_name);
            let start = Instant::now();
            let success = downloader.download(&movie_url, &filename_str).await?;
            let time_taken = start.elapsed().as_secs();

            if success {
                info!(
                    "Successfully downloaded the movie [{}]. Took [{}] seconds!",
                    movie_name, time_taken
                );
            }
        } else {
            info!(
                "Skipping downloading the movie {} since the option \"--skip-download\" is enabled.",
                movie_name
            );
        }
    } else {
        info!("{} already downloaded", filename_str);
    }

    Ok(())
}

async fn print_movie_plot(client: &EinthusanClient, movie_page_url: &str) -> Result<(), AppError> {
    if let Some(plot) = client.fetch_movie_plot(movie_page_url).await? {
        println!("{plot}");
    } else {
        info!("No plot was found for {}", movie_page_url);
    }

    Ok(())
}
