use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "einthusan-dl",
    about = "Download movie from Einthusan.",
    version = concat!(env!("CARGO_PKG_VERSION"), " (default parallel downloads: 1)")
)]
pub struct Args {
    /// URL(s) of the movie to be downloaded
    #[arg(required = true)]
    pub urls: Vec<String>,

    /// Whether existing files should be overwritten (default: False)
    #[arg(short = 'o', long = "overwrite", default_value_t = false)]
    pub overwrite: bool,

    /// Logs to the specified logfile (default: logs to console)
    #[arg(short = 'l', long = "log")]
    pub logfile: Option<String>,

    /// Use wget for downloading, optionally specify wget bin
    #[arg(long = "wget", num_args = 0..=1, default_missing_value = "wget")]
    pub wget: Option<String>,

    /// Use curl for downloading, optionally specify curl bin
    #[arg(long = "curl", num_args = 0..=1, default_missing_value = "curl")]
    pub curl: Option<String>,

    /// For debugging: skip actual downloading of files
    #[arg(long = "skip-download", default_value_t = false)]
    pub skip_download: bool,

    /// Path to save the file
    #[arg(long = "path", default_value = "")]
    pub path: String,

    /// Write the movie name to the specified file
    #[arg(long = "output-movie-name")]
    pub output_movie_name: Option<String>,

    /// Print the movie plot
    #[arg(long = "plot", default_value_t = false)]
    pub plot: bool,

    /// Print lots of debug information
    #[arg(long = "debug", default_value_t = false)]
    pub debug: bool,
}
