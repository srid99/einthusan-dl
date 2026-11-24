use einthusan_dl::scraper::{decode_links, parse_movie_page};
use std::time::Instant;

#[test]
fn bench_decoder_throughput() {
    let raw_encoded = "aHR0cHM6Ly9leGFtcGxlLmNvbS92aWRlby5tcDQ=";
    let encoded_str = format!("foo10bar{}baz", raw_encoded);

    let iterations = 100_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = decode_links(&encoded_str);
    }
    let elapsed = start.elapsed();
    let per_op_ns = elapsed.as_nanos() as f64 / iterations as f64;
    let per_op_us = per_op_ns / 1000.0;
    println!(
        "Rust decode_links: total={:?}, per_op={:.3} µs ({} iterations)",
        elapsed, per_op_us, iterations
    );
}

#[test]
fn bench_dom_parsing() {
    let fixture = include_str!("../tests/fixtures/watch_page_normal.html");
    let iterations = 100;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = parse_movie_page(fixture);
    }
    let elapsed = start.elapsed();
    let per_op_ms = elapsed.as_secs_f64() * 1000.0 / iterations as f64;
    println!(
        "Rust parse_movie_page: total={:?}, per_op={:.3} ms ({} iterations)",
        elapsed, per_op_ms, iterations
    );
}
