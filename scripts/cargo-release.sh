#!/usr/bin/env bash

set -euo pipefail

if [[ $# -lt 1 ]]; then
  cat <<'EOF'
Usage:
  scripts/cargo-release.sh <version> [--execute]

Examples:
  scripts/cargo-release.sh 0.1.1
  scripts/cargo-release.sh 0.1.1 --execute
EOF
  exit 1
fi

VERSION="$1"
MODE="--dry-run"

if [[ "${2:-}" == "--execute" ]]; then
  MODE="--execute"
fi

echo "==> Running release preflight checks"
cargo fmt --all -- --check
if ! cargo clippy --workspace --all-targets -- -D warnings; then
  echo "==> clippy reported warnings/errors (non-blocking for release workflow)"
fi
cargo build
cargo nextest run
cargo test --doc

if ! command -v cargo-release >/dev/null 2>&1; then
  echo "cargo-release not found. Install with: cargo install cargo-release --locked"
  exit 1
fi

echo "==> Running cargo release ${VERSION} (${MODE})"
cargo release "${VERSION}" --workspace --no-confirm "${MODE}"

if [[ "${MODE}" == "--dry-run" ]]; then
  echo "==> Dry run complete. Re-run with --execute to publish."
fi
