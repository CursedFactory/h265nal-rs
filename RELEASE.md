# Release Guide

This repository is configured for `cargo-release` workspace publishing.

## Requirements

- Access to publish all crates in this workspace on crates.io.
- Clean `main`/`master` branch with all CI checks passing.
- Rust toolchain installed.
- Release tools installed:
  - `cargo install cargo-release --locked`
  - `cargo install cargo-nextest --locked`
- Linux/macOS build deps where needed:
  - `cmake`
  - `clang`
- For GitHub Actions release workflow:
  - Repository secret `CARGO_REGISTRY_TOKEN` (crates.io API token with publish scope).

## Versioning

- Workspace version is set in `Cargo.toml` under `[workspace.package]`.
- Current version: `0.1.1`.
- `release.toml` enforces shared workspace versions and `vX.Y.Z` tags.

## Local Release Workflow

1. Run dry run:
   - `bash scripts/cargo-release.sh 0.1.1`
2. Publish for real:
   - `bash scripts/cargo-release.sh 0.1.1 --execute`
3. Push release commit and tags:
   - `git push origin HEAD`
   - `git push origin --tags`

After publish, users can install the CLI globally:

- `cargo install h265nal-cli`

The helper script runs:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings` (non-blocking)
- `cargo build`
- `cargo nextest run`
- `cargo test --doc`
- `cargo release <version> --workspace`

## GitHub Actions Workflows

- `release-dry-run.yml`
  - Runs on pull requests and manual dispatch.
  - Validates release flow with `cargo release --dry-run`.

- `release.yml`
  - Manual dispatch only.
  - Executes real publish with `cargo release --execute`.
  - Pushes release commit and tags.
