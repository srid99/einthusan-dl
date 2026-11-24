use std::fs::OpenOptions;
use std::path::Path;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::error::AppError;

pub fn init_logging(log_file: Option<&str>, debug: bool) -> Result<(), AppError> {
    let level = if debug { Level::DEBUG } else { Level::INFO };
    if let Some(path) = log_file {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(path))
            .map_err(AppError::Io)?;

        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .with_writer(file)
            .with_target(false)
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);
    } else {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .with_writer(std::io::stdout)
            .with_target(false)
            .without_time()
            .finish();

        let _ = tracing::subscriber::set_global_default(subscriber);
    }

    Ok(())
}
