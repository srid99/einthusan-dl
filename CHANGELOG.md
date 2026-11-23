# Changelog

All notable changes to this project will be documented in this file.

## [2.0.1] - 2026-11-22

- Parallel downloads: split each movie into four byte-range fragments downloaded concurrently.
- Retry failed fragments individually instead of restarting the whole download.
- Show per-fragment download progress.
- Add `--version` flag.
- Security: update `url` to 2.5.9 (fixes RUSTSEC-2026-0592).

[2.0.1]: https://github.com/sunilsankar/einthusan-dl/compare/v2.0.0...v2.0.1
