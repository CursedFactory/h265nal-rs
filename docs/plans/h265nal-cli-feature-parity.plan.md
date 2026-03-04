# H265NAL CLI Feature Parity Plan

## Goal
- Expand `crates/h265nal-parity` from MVP-only checks into a full feature-tracking parity matrix against `tools/h265nal.cc`.
- Keep a stable inventory of feature coverage, known gaps, and regressions with reproducible command outputs.

## Workflow
- Use `h265nal-parity list` to inspect available scenarios and expectation labels (`must_match` vs `known_gap`).
- Use `h265nal-parity compare --scenario <name>` for focused debugging and unified diff output.
- Use `h265nal-parity compare-all` to run the full matrix and fail only on unexpected regressions in `must_match` scenarios.
- Use `h265nal-parity report --output <path>` to write a markdown report with summary tables and diff excerpts.

## Scenario Coverage Snapshot
- Total scenarios: 31
- `must_match` scenarios: 12
- `known_gap` scenarios: 19
- Feature inventory coverage: 26/26 CLI features (`tools/h265nal.cc` + missing-input behavior)

## Known Gap Buckets
- `--dump-length` and `--frames-per-second` execution path is not implemented in Rust CLI.
- `--nalu-length-bytes` modes other than `-1` are not implemented in Rust CLI.
- Metadata toggles (`--add-offset`, `--add-length`, `--add-parsed-length`, `--add-checksum`, `--add-resolution`, `--add-contents`) are not yet emitted by Rust C-mode dump output.
- `--outfile <file>` with `--dump-all` is not yet supported in Rust CLI.
- `--hvcc-file` parity is partial (`hvcc` priming and hvcc-only parse path are not implemented).
- `--help` and `--version` output format currently follows clap defaults, not native C++ formatting.
- `-d` verbosity semantics differ (`bool` in Rust CLI vs counted debug in C++ CLI).

## Canonical Commands
```bash
# Single scenario
cargo run -p h265nal-parity -- compare --scenario dump_one_line

# Full matrix, regression gate on must-match scenarios
cargo run -p h265nal-parity -- compare-all

# Full markdown report with command mapping + diff snippets
cargo run -p h265nal-parity -- report --output docs/plans/h265nal-cli-feature-parity.report.md
```

## CI Integration
- `nextest_by_crate` now enables docker parity tests for `h265nal-parity` via `H265NAL_PARITY_RUN_DOCKER_TESTS=1`.
- Integration test `crates/h265nal-parity/tests/parity_video.rs` runs end-to-end `dump_one_line` and `dump_multiline` parity checks across every `.265` fixture in `video/`.
- CI runs `cargo run -p h265nal-parity -- report --output parity-report.md` and appends the report markdown into GitHub step summary.
- CI keeps the parity output inline in the GitHub step summary (no artifact upload step).

## Run Log
| Date (UTC) | Scope | Command | Outcome | Notes |
|---|---|---|---|---|
| 2026-03-03 | Matrix expansion | `cargo nextest run -p h265nal-parity` | Pass | Added 31-scenario matrix definitions, feature coverage checks, and report generation module. |
| 2026-03-03 | CI + video e2e | `.github/workflows/ci.yml` parity row | Pending CI run | Added docker-enabled parity tests over all `video/*.265` fixtures and summary report publication. |
