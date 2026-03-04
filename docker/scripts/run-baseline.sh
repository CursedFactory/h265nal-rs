#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
ROOT_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)

exec docker compose -f "$ROOT_DIR/docker/compose.yml" run --rm -T baseline -- "$@"
