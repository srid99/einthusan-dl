use std::io::{self, Write};
use std::time::Instant;

pub fn format_bytes(bytes: Option<u64>) -> String {
    let bytes = match bytes {
        Some(b) => b as f64,
        None => return "N/A".to_string(),
    };

    if bytes == 0.0 {
        return "0.00B".to_string();
    }

    let exponent = (bytes.ln() / 1024.0_f64.ln()).floor() as usize;
    let suffixes = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let exp = exponent.min(suffixes.len() - 1);
    let converted = bytes / (1024.0_f64.powi(exp as i32));
    format!("{:.2}{}", converted, suffixes[exp])
}

pub struct DownloadProgress {
    total: Option<u64>,
    current: u64,
    start: Option<Instant>,
    now: Option<Instant>,
    finished: bool,
}

impl DownloadProgress {
    pub fn new(total: Option<u64>) -> Self {
        let total = match total {
            Some(0) => None,
            other => other,
        };

        Self {
            total,
            current: 0,
            start: None,
            now: None,
            finished: false,
        }
    }

    pub fn start(&mut self) {
        let now = Instant::now();
        self.start = Some(now);
        self.now = Some(now);
    }

    pub fn read(&mut self, bytes: u64) {
        self.current += bytes;
        let now = Instant::now();
        let should_report = match self.now {
            Some(last) => now.duration_since(last).as_millis() >= 100,
            None => true,
        };
        if should_report {
            self.now = Some(now);
            self.report_progress();
        }
    }

    pub fn stop(&mut self) {
        self.now = Some(Instant::now());
        self.finished = true;
        self.total = Some(self.current);
        self.report_progress();
    }

    fn calc_percent(&self) -> String {
        let total = match self.total {
            Some(t) if t > 0 => t,
            _ => return "--%".to_string(),
        };

        let percentage = ((self.current as f64 / total as f64) * 100.0) as u64;
        let done = (percentage / 2) as usize;
        let hashes = "#".repeat(done);
        format!("[{: <50}] {}%", hashes, percentage)
    }

    fn calc_speed(&self) -> String {
        let start = match self.start {
            Some(s) => s,
            None => return "---b/s".to_string(),
        };
        let now = self.now.unwrap_or_else(Instant::now);
        let dif = now.duration_since(start).as_secs_f64();

        if self.current == 0 || dif < 0.001 {
            return "---b/s".to_string();
        }

        let bytes_per_sec = (self.current as f64 / dif) as u64;
        format!("{}/s", format_bytes(Some(bytes_per_sec)))
    }

    pub fn report_progress(&self) {
        let percent = self.calc_percent();
        let total = format_bytes(self.total);
        let speed = self.calc_speed();
        let total_speed_report = format!("{} at {}", total, speed);

        let report = format!("\r{: <56} {: >30}", percent, total_speed_report);
        let mut stdout = io::stdout();
        if self.finished {
            let _ = writeln!(stdout, "{}", report);
        } else {
            let _ = write!(stdout, "{}", report);
        }
        let _ = stdout.flush();
    }
}
