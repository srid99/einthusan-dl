use einthusan_dl::scraper::{parse_movie_page, parse_movie_page_with_plot};

const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");

#[test]
fn test_parse_normal_watch_page() {
    let path = format!("{}/watch_page_normal.html", FIXTURES_DIR);
    let html = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {path}: {e}"));

    let meta = parse_movie_page_with_plot(&html).expect("Failed to parse normal watch page");
    assert!(!meta.page_id.is_empty());
    assert!(!meta.ejpingables.is_empty());
    assert_eq!(meta.movie_name, "Jailer 2");
    assert_eq!(
        meta.plot.as_deref(),
        Some("A retired officer's peaceful life is disrupted when a dangerous criminal organization threatens his family. He must use his skills one final time to protect his loved ones and preserve his legacy.")
    );
}

#[test]
fn test_parse_missing_player_page() {
    let path = format!("{}/watch_page_missing_player.html", FIXTURES_DIR);
    let html = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {path}: {e}"));

    let result = parse_movie_page(&html);
    assert!(result.is_err());
    let err_str = result.unwrap_err().to_string();
    assert!(err_str.contains("Missing section#UIVideoPlayer"));
}

#[test]
fn test_parse_malformed_page() {
    let path = format!("{}/watch_page_malformed.html", FIXTURES_DIR);
    let html = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {path}: {e}"));

    let result = parse_movie_page(&html);
    assert!(result.is_err());
    let err_str = result.unwrap_err().to_string();
    assert!(err_str.contains("Missing data-ejpingables"));
}
