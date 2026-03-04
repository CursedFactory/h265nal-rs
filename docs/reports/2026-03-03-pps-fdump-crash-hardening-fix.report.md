# Report: pps_fdump_crash Hardening Fix

## Metadata
- Date: 2026-03-03
- Scope: native no-segfault hardening for `--dump` on `media/pps_fdump_crash.202203.265`
- Status: completed

## Goal
- Keep the wrapper thin while preventing native crash paths from terminating process execution.

## Code Changes
- `src/h265_pps_parser.cc`
  - In parse path, after `ParsePpsMultilayerExtension(...)`, added null check and early `return nullptr` when extension parse fails.
  - In `fdump`, added pointer guard before `pps_multilayer_extension->fdump(...)` and error log on missing state.
- `crates/h265nal-cli/tests/smoke.rs`
  - Replaced known-failure ignored regression with active regression:
    - `dump_regression_pps_fdump_crash_no_segfault`
  - New expectation: command succeeds and does not segfault on crash fixture.

## Why This Fix
- Parser had an inconsistent state path:
  - extension flag set,
  - extension pointer null,
  - dump path dereferenced null pointer.
- This fix converts that into standard parser failure for affected NAL units and avoids dereferencing null state in dump.

## Validation

### Rust smoke suite
- `cargo nextest run -p h265nal-sys -p h265nal-cli` -> pass (6/6)

### Direct C ABI (no Rust wrapper)
- Rebuilt harness:
```bash
clang -std=c11 -Iinclude docs/reports/repro_h265_c_api_dump.c \
  target/debug/build/h265nal-sys-c7fc46a5de476cd3/out/build/src/libh265nal.a \
  -lc++ -o target/repro_h265_c_api_dump
```
- Results:
  - `target/repro_h265_c_api_dump media/nvenc.265` -> `0`
  - `target/repro_h265_c_api_dump media/pps_fdump_crash.202203.265` -> `0`

### Upstream C++ tool
- Rebuilt tool:
```bash
cmake --build target/debug/build/h265nal-sys-c7fc46a5de476cd3/out/build --target h265nal-bin
```
- Results:
  - `.../out/build/tools/h265nal -i media/nvenc.265` -> `0`
  - `.../out/build/tools/h265nal -i media/pps_fdump_crash.202203.265` -> `0`

## Observed Behavior After Fix
- No segfault in Rust CLI, direct C ABI harness, or upstream C++ tool on crash fixture.
- Parser still logs unsupported extension messages (expected current parser capability limits).
- Dump returns success and emits available parsed NAL output without process crash.

## Follow-up
1. If desired, tighten status semantics so unsupported parse branches propagate a non-zero status from C ABI dump.
2. Add explicit fixture assertion for stderr diagnostics if we want deterministic unsupported-feature reporting.
