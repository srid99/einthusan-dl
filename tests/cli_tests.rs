use clap::Parser;
use einthusan_dl::cli::Args;

#[test]
fn test_cli_minimal_args() {
    let args = Args::try_parse_from(["einthusan-dl", "https://einthusan.tv/movie/watch/2ZEY/"])
        .expect("Failed to parse minimal args");

    assert_eq!(args.urls, vec!["https://einthusan.tv/movie/watch/2ZEY/"]);
    assert!(!args.overwrite);
    assert!(!args.skip_download);
    assert!(!args.plot);
    assert_eq!(args.path, "");
    assert!(args.wget.is_none());
    assert!(args.curl.is_none());
    assert!(args.logfile.is_none());
}

#[test]
fn test_cli_all_flags() {
    let args = Args::try_parse_from([
        "einthusan-dl",
        "https://einthusan.tv/movie/watch/2ZEY/",
        "-o",
        "--skip-download",
        "--path",
        "/tmp/movies",
        "--log",
        "/tmp/einthusan.log",
        "--wget",
        "/usr/local/bin/wget",
        "--output-movie-name",
        "/data/einthusan-dl/test.txt",
        "--plot",
    ])
    .expect("Failed to parse all flags");

    assert!(args.overwrite);
    assert!(args.skip_download);
    assert!(args.plot);
    assert_eq!(args.path, "/tmp/movies");
    assert_eq!(args.logfile.as_deref(), Some("/tmp/einthusan.log"));
    assert_eq!(args.wget.as_deref(), Some("/usr/local/bin/wget"));
    assert_eq!(
        args.output_movie_name.as_deref(),
        Some("/data/einthusan-dl/test.txt")
    );
}

#[test]
fn test_cli_default_wget_and_curl_bins() {
    let args_wget = Args::try_parse_from(["einthusan-dl", "https://test.com", "--wget"])
        .expect("Failed to parse --wget with default");
    assert_eq!(args_wget.wget.as_deref(), Some("wget"));

    let args_curl = Args::try_parse_from(["einthusan-dl", "https://test.com", "--curl"])
        .expect("Failed to parse --curl with default");
    assert_eq!(args_curl.curl.as_deref(), Some("curl"));
}

#[test]
fn test_cli_version_includes_release_and_parallel_default() {
    let error = Args::try_parse_from(["einthusan-dl", "--version"])
        .expect_err("--version should exit without requiring a movie URL");
    let output = error.to_string();

    assert!(output.contains("einthusan-dl 2.0.0-beta"));
    assert!(output.contains("default parallel downloads: 4"));
}
