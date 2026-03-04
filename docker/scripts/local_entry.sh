#!/bin/sh
set -eu

fail() {
  printf 'local_entry.sh: %s\n' "$1" >&2
  exit 1
}

WORK_DIR=${WORK_DIR:-/work}
CARGO_HOME=${CARGO_HOME:-/opt/local-cargo-home}
CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/opt/local-target}

[ -f "$WORK_DIR/Cargo.toml" ] || fail "missing Cargo.toml in /work"

mkdir -p "$CARGO_HOME" "$CARGO_TARGET_DIR"

cd "$WORK_DIR"

cargo build -p h265nal-cli --locked --target-dir "$CARGO_TARGET_DIR" || fail "cargo build failed"

BIN_PATH="$CARGO_TARGET_DIR/debug/h265nal-cli"
[ -x "$BIN_PATH" ] || fail "expected binary missing: $BIN_PATH"

if [ "${1:-}" = "--" ]; then
  shift
fi

exec "$BIN_PATH" "$@"
