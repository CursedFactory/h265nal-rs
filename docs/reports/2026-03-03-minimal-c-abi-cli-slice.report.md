# Report: Minimal C ABI + Rust CLI Slice

## Metadata
- Date: 2026-03-03
- Scope plan: `docs/plans/h265nal-c-abi-cli-minimal.plan.md`
- Status: implemented and passing smoke gate

## Related Reports
- Native isolation details for crash fixture:
  - `docs/reports/2026-03-03-pps-fdump-crash-native-isolation.report.md`
- Native hardening fix details:
  - `docs/reports/2026-03-03-pps-fdump-crash-hardening-fix.report.md`

## Implemented Work

### 1) Native C ABI shim
- Added C header: `include/h265_c_api.h`
- Added shim implementation: `src/h265_c_api.cc`
- Exported symbols:
  - `h265nal_annexb_count_nalus`
  - `h265nal_annexb_dump`
- Status and flag constants:
  - `H265NAL_STATUS_OK`
  - `H265NAL_STATUS_INVALID_ARGUMENT`
  - `H265NAL_STATUS_PARSE_FAILURE`
  - `H265NAL_STATUS_DUMP_UNAVAILABLE`
  - `H265NAL_DUMP_FLAG_ONE_LINE`
- Wired shim into both CMake library source lists in `src/CMakeLists.txt`.

### 2) Rust FFI surface (`h265nal-sys`)
- Implemented extern declarations and wrappers in `crates/h265nal-sys/src/lib.rs`.
- Added error mapping enum:
  - `InvalidArgument`
  - `ParseFailure`
  - `DumpUnavailable`
  - `UnknownStatus(i32)`
- Added minimal wrapper API:
  - `count_nalus_annexb(data: &[u8]) -> Result<usize, Error>`
  - `dump_annexb_to_stdout(data: &[u8], flags: u32) -> Result<(), Error>`

### 3) Minimal CLI crate (`h265nal-cli`)
- Added workspace member in `Cargo.toml`.
- Added crate manifest: `crates/h265nal-cli/Cargo.toml`.
- Added CLI implementation: `crates/h265nal-cli/src/main.rs`.
- Supported modes:
  - `h265nal-cli <input.265>` -> prints `nal_units=<count>`
  - `h265nal-cli --dump <input.265>` -> dumps parsed NAL units to stdout
  - `h265nal-cli --dump --one-line <input.265>` -> one-line dump mode via `DUMP_FLAG_ONE_LINE`

### 4) Smoke tests (nextest)
- Sys tests: `crates/h265nal-sys/tests/smoke.rs`
  - count smoke: `pps_fdump_crash.202203.265`
  - dump smoke: `nvenc.265`
- CLI tests: `crates/h265nal-cli/tests/smoke.rs`
  - default count mode output contains `nal_units=`
  - dump mode output contains `nal_unit {`
  - dump one-line mode output contains `nal_unit {`
- Added ignored regression test for known crash fixture:
  - `dump_regression_pps_fdump_crash_known_failure`
  - marked `#[ignore]` with explicit known-crash reason

## Validation Results
- `cargo build` -> pass
- `cargo build -p h265nal-sys -vv` -> pass
- `cargo nextest run -p h265nal-sys -p h265nal-cli` -> pass (5 passed, 1 skipped)
- `cargo test -p h265nal-cli --test smoke -- --ignored dump_regression_pps_fdump_crash_known_failure` -> pass

## Issue Investigation: `pps_fdump_crash.202203.265` dump segfault

### Symptom
- `h265nal-cli --dump media/pps_fdump_crash.202203.265` exits with signal 11 (segfault).

### Repro
```bash
cargo run -p h265nal-cli -- --dump media/pps_fdump_crash.202203.265
```

### Debug evidence
- LLDB shows crash in:
  - `src/h265_pps_multilayer_extension_parser.cc:210`
  - `h265nal::H265PpsMultilayerExtensionParser::PpsMultilayerExtensionState::fdump`
- Crash frame has `this = 0x0` (null dereference).

### Likely root cause chain
1. Parse path can return null multilayer extension state when unsupported feature is present:
   - `colour_mapping_enabled_flag` case returns `nullptr` in `src/h265_pps_multilayer_extension_parser.cc:188`.
2. PPS parser stores parser output without null validation:
   - assignment in `src/h265_pps_parser.cc:303`.
3. Dump path dereferences `pps_multilayer_extension` when the flag is set, without pointer check:
   - `src/h265_pps_parser.cc:528` then calls `fdump` on null pointer.

### Current mitigation in this slice
- Keep default dump smoke tests on stable fixture `media/nvenc.265`.
- Preserve crash fixture coverage as ignored regression test documenting known behavior.

### Rust-wrapper isolation check
- Objective: rule out Rust wrapper bugs and verify whether crash is in native layer.
- Added direct C harness: `docs/reports/repro_h265_c_api_dump.c`.
- Built harness against static native library from `h265nal-sys` build output.

Observed return codes:
- Direct C ABI call (`h265nal_annexb_dump`) on `media/nvenc.265` -> `0`
- Direct C ABI call (`h265nal_annexb_dump`) on `media/pps_fdump_crash.202203.265` -> `-11` (SIGSEGV)
- Upstream C++ tool `h265nal` on `media/nvenc.265` -> `0`
- Upstream C++ tool `h265nal` on `media/pps_fdump_crash.202203.265` -> `-11` (SIGSEGV)

Conclusion:
- Crash reproduces in direct C ABI and upstream C++ tool paths.
- This rules out the Rust wrapper as primary cause; fault is in native parse/dump behavior.

## Recommended Follow-up
1. Add native guard in PPS dump path: check `pps_multilayer_extension != nullptr` before `fdump`.
2. Consider parse-time policy for unsupported multilayer extensions:
   - either fail entire PPS parse consistently,
   - or mark partial parse and avoid dereference in dump.
3. Add a non-ignored regression test once native hardening converts crash into deterministic error.
