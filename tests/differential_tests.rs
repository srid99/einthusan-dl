use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn test_diff_010_help_flag() {
    let mut cmd = Command::cargo_bin("einthusan-dl").expect("binary should exist");
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Download movie from Einthusan."))
        .stdout(predicate::str::contains("--skip-download"))
        .stdout(predicate::str::contains("--overwrite"));
}

#[test]
fn test_diff_008_invalid_url_exits_cleanly() {
    let mut cmd = Command::cargo_bin("einthusan-dl").expect("binary should exist");
    // DEC-001: Process exits with 0 even when single movie fails to download
    cmd.arg("https://127.0.0.1:9/invalid-movie-url");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Failed to process movie"));
}

#[test]
fn test_diff_002_custom_path_creation() {
    let temp = tempdir().expect("create temp dir");
    let target_path = temp.path().join("downloads");

    let mut cmd = Command::cargo_bin("einthusan-dl").expect("binary should exist");
    cmd.args([
        "https://127.0.0.1:9/fake-url",
        "--path",
        target_path.to_str().unwrap(),
        "--skip-download",
    ]);

    // Exits cleanly
    cmd.assert().success();
}
