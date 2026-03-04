# Report: Native Suite Wrapper Pass 1

## Metadata
- Date: 2026-03-03
- Scope: `docs/plans/h265nal-rust-native-suite-wrapper.plan.md` (Phase 0) + `docs/plans/h265nal-parity-matrix.todo.md` (Track C foundation)
- Status: completed for this pass

## Executive Summary
- Added an out-of-band Rust crate, `h265nal-native-suite`, that wraps the native `test/*.cc` inventory and provides a `cargo nextest` entrypoint for migration tracking without impacting default workspace loops.
- Verified staged Docker A/B comparisons against baseline C++ and local Rust via existing parity scenarios.

## Progress & Checklist Alignment
- [x] Added wrapper crate and workspace wiring while keeping it out of `default-members`.
- [x] Added docs-index inventory parser and coverage checks (59 cases from `docs/test-cases/README.md`, 28 native `_unittest.cc` files).
- [x] Added wrapper status tracking (`inventory-only` vs docker-scenario proxy).
- [x] Added file-family wrapper tests covering all 28 native `_unittest.cc` files.
- [x] Added staged docker compare tests (`dump_one_line`, `dump_multiline`, `dump_crash_fixture`).
- [x] Updated `docs/plans/h265nal-parity-matrix.todo.md` with Track C section and gates.
- [x] Added dedicated plan: `docs/plans/h265nal-rust-native-suite-wrapper.plan.md`.

## Engineering Log
- [DECISION] Create `crates/h265nal-native-suite` as a separate workspace member and set workspace `default-members` to existing main crates (`h265nal-sys`, `h265nal-cli`, `h265nal-parity`) so day-to-day main crate loops stay fast.
- [DECISION] Use `docs/test-cases/README.md` as the canonical case index for wrapper coverage tracking, and validate it against live native `test/*_unittest.cc` discovery.
- [HACK] Case wrappers are currently inventory-first: most wrappers are not yet executable per-case parity checks. Tech Debt: map each wrapper case to concrete parity scenarios and migrate from proxy/inventory status to executable status.
- [HACK] Two case wrappers currently use proxy docker scenario mapping (`TestSampleBitstream`, `TestSampleBitstreamAlt`) for staged signal only, not strict 1:1 semantic case equivalence. Tech Debt: replace with direct family/case mappings.
- [BLOCKER] Full per-case executable parity cannot proceed until Track B expands scenario coverage (especially mode/input/output semantics and diff harness breadth). Requires: additional `crates/h265nal-parity` scenarios and corresponding CLI behavior parity.

## Changes

### Code
- `Cargo.toml`: added `crates/h265nal-native-suite` as workspace member and added explicit `default-members` excluding this out-of-band suite.
- `crates/h265nal-native-suite/Cargo.toml`: new crate manifest.
- `crates/h265nal-native-suite/src/lib.rs`: public API exports for inventory and wrapper runner utilities.
- `crates/h265nal-native-suite/src/inventory.rs`: parser/loader for docs index + native file inventory checks.
- `crates/h265nal-native-suite/src/suite.rs`: wrapper status model and staged docker scenario execution helpers.
- `crates/h265nal-native-suite/tests/native_inventory.rs`: inventory consistency tests.
- `crates/h265nal-native-suite/tests/native_wrappers.rs`: wrapper coverage/status tests.
- `crates/h265nal-native-suite/tests/native_file_suite.rs`: file-family wrapper coverage tests (28 native files).
- `crates/h265nal-native-suite/tests/docker_stage.rs`: env-gated docker A/B stage checks.

### Docs
- `docs/plans/h265nal-rust-native-suite-wrapper.plan.md`: new sequential plan for the wrapper crate.
- `docs/plans/h265nal-parity-matrix.todo.md`: new Track C section and gates.

## Verification Evidence

### Test Suite Status
```bash
$ cargo nextest run -p h265nal-native-suite
Summary: 41 tests run: 41 passed, 0 skipped
```

### Docker A/B Stage Check (baseline C++ vs local Rust)
```bash
$ H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo nextest run -p h265nal-native-suite
Summary: 41 tests run: 41 passed, 0 skipped
```

### Cross-Crate Validation From This Sequence
```bash
$ cargo nextest run -p h265nal-sys -p h265nal-cli -p h265nal-native-suite
Summary: 63 tests run: 63 passed, 0 skipped
```

## Continuation Prompt
Use this in the next session:

"Continue from `docs/reports/2026-03-03-native-suite-wrapper-pass1.report.md` and `docs/plans/h265nal-rust-native-suite-wrapper.plan.md`. Keep the thin-wrapper/1:1/A-B-diff goals strict. Next priorities: (1) expand Track B CLI parity semantics (`dump-length`, `nalu-length-bytes`, `frames-per-second`) with docker-verified scenarios, (2) add corresponding scenarios in `crates/h265nal-parity`, (3) convert `h265nal-native-suite` wrappers from inventory/proxy status to executable per-case mappings in dependency order (core/shared -> parameter sets -> payload -> RTP), and (4) update `docs/plans/h265nal-parity-matrix.todo.md` after each mapping slice with test evidence." 
