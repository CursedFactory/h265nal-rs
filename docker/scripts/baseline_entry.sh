#!/bin/sh
set -eu

fail() {
  printf 'baseline_entry.sh: %s\n' "$1" >&2
  exit 1
}

UPSTREAM_REPO_URL=${UPSTREAM_REPO_URL:-https://github.com/chemag/h265nal}
UPSTREAM_REF=${UPSTREAM_REF:-main}

[ -n "$UPSTREAM_REF" ] || fail "UPSTREAM_REF is empty"
[ -n "$UPSTREAM_REPO_URL" ] || fail "UPSTREAM_REPO_URL is empty"

SRC_DIR=${BASELINE_SRC_DIR:-/opt/baseline-src}
BUILD_DIR=${BASELINE_BUILD_DIR:-/opt/baseline-build}

if [ ! -d "$SRC_DIR/.git" ]; then
  mkdir -p "$SRC_DIR"
  git clone "$UPSTREAM_REPO_URL" "$SRC_DIR" || fail "clone failed"
fi

mkdir -p "$BUILD_DIR"

git -C "$SRC_DIR" remote set-url origin "$UPSTREAM_REPO_URL" || fail "set remote failed"

git -C "$SRC_DIR" fetch --tags origin || fail "git fetch failed"

if git -C "$SRC_DIR" rev-parse --verify --quiet "$UPSTREAM_REF^{commit}" >/dev/null; then
  CHECKOUT_REF=$UPSTREAM_REF
else
  git -C "$SRC_DIR" fetch origin "$UPSTREAM_REF" || fail "cannot fetch ref: $UPSTREAM_REF"
  CHECKOUT_REF=FETCH_HEAD
fi

git -C "$SRC_DIR" checkout --detach "$CHECKOUT_REF" || fail "checkout failed: $UPSTREAM_REF"

cmake -S "$SRC_DIR" -B "$BUILD_DIR" -DCMAKE_BUILD_TYPE=Release || fail "cmake configure failed"
cmake --build "$BUILD_DIR" --target h265nal -- -j"$(getconf _NPROCESSORS_ONLN 2>/dev/null || printf 2)" \
  || fail "cmake build failed for target h265nal"

BIN_PATH="$BUILD_DIR/tools/h265nal"
[ -x "$BIN_PATH" ] || fail "expected binary missing: $BIN_PATH"

if [ "${1:-}" = "--" ]; then
  shift
fi

exec "$BIN_PATH" "$@"
