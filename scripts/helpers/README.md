# Notes helper scripts (Bun)

These scripts replace the legacy Python helpers under `scripts/helpers/python/`.

## Usage pattern

- All executable scripts use `#!/usr/bin/env bun`.
- Run with `bun <script> [--dir <notes-dir>]`.
- If `--dir` is omitted, scripts try:
  1. `H265NAL_MAP_DIR`
  2. `<repo>/notes/h265nal-rs-map`

If no usable directory is found, the script exits with a message telling you to pass `--dir`.

## Locations

- Shared utilities: `scripts/helpers/notes/common.ts`
- Migrated scripts: `scripts/helpers/notes/*.sh.ts`

## Quick examples

- `bun scripts/helpers/notes/check_links.sh.ts --dir /path/to/h265nal-rs-map`
- `bun scripts/helpers/notes/check_headings.sh.ts --dir /path/to/h265nal-rs-map`
- `bun scripts/helpers/notes/normalize_notes.sh.ts --dir /path/to/h265nal-rs-map`
