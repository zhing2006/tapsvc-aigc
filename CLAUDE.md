# CLAUDE.md

## Build & Check

```bash
cargo build
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
```

All code must pass `cargo fmt` and `cargo clippy` before commit.

## Project Structure

Cargo workspace with 3 crates:
- `crates/tapsvc-aigc` — CLI binary
- `crates/tapsvc-aigc-openai` — OpenAI compatible API client library
- `crates/tapsvc-aigc-ark` — Volcengine ARK API client library

## Dependencies

- All dependencies use `default-features = false` with manually selected features
- Versions managed in workspace root `Cargo.toml`
- Target: static linking, single binary distribution

## Environment

- `TAPSVC_BASE_URL` — API proxy base URL
- `TAPSVC_API_KEY` — API key
- Loaded via `.env` file (dotenvy) or environment variables
