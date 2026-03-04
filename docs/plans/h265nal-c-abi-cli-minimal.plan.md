# h265nal Minimal C ABI + Rust CLI Plan

## Goal
Ship the smallest useful runtime vertical slice:

1) expose the C++ parser through a C ABI shim,
2) add first Rust FFI symbols in `h265nal-sys`,
3) add a Rust CLI crate with `count` and `--dump`,
4) cover the slice with minimal `cargo nextest run` smoke tests.

This pass optimizes for integration speed and deterministic testing over API breadth.

## Status
- Baseline (`h265nal-sys` build plumbing via CMake): complete.
- Iteration 1 (C ABI shim): pending.
- Iteration 2 (Rust FFI + wrappers): pending.
- Iteration 3 (CLI crate + tests): pending.
- Iteration 4 (workspace smoke gate): pending.

## Scope (This Pass)
- Add minimal C ABI shim files (`include/` + `src/`) and compile into `h265nal`.
- Expose two initial FFI calls in `crates/h265nal-sys`.
- Add one Rust binary crate at `crates/h265nal-cli`.
- Add minimal nextest-compatible tests for sys + cli crates.

## Non-Goals (Deferred)
- Full `tools/h265nal` option parity.
- Bindgen-generated API surface.
- A high-level safe wrapper crate (`crates/h265nal`) beyond tiny helpers in `-sys`.
- NALU-length framing mode wrappers (`ParseBitstreamNALULength`).

## Locked Decisions For This Slice
- Annex-B byte stream only.
- Handwritten FFI surface (no bindgen in this pass).
- Dump path writes directly to `stdout` (no cross-language output buffer yet).
- No cross-language memory ownership transfer.
- Unknown dump-flag bits are ignored for forward compatibility.

## Research Summary
- Best count entrypoint: `H265BitstreamParser::FindNaluIndices(...)` (`include/h265_bitstream_parser.h:73`).
- Best dump entrypoint: `ParseBitstream(...)` + per-NAL `fdump(...)` (`tools/h265nal.cc:471`).
- `fdump` is gated by `FDUMP_DEFINE`; normal footprint enables it, small footprint disables it (`src/CMakeLists.txt:11`, `src/CMakeLists.txt:16`).
- Shim integration is straightforward by adding one source file to both `add_library(h265nal ...)` source lists (`src/CMakeLists.txt:17`, `src/CMakeLists.txt:48`).
- For CLI integration tests under nextest, use `CARGO_BIN_EXE_h265nal-cli`.
- Fast local fixture: `media/pps_fdump_crash.202203.265`.

## Minimal C ABI Contract

### File Placement
- New header: `include/h265_c_api.h`
- New source: `src/h265_c_api.cc`

### Proposed Header Shape
```c
#ifndef H265_C_API_H_
#define H265_C_API_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

enum {
  H265NAL_STATUS_OK = 0,
  H265NAL_STATUS_INVALID_ARGUMENT = 1,
  H265NAL_STATUS_PARSE_FAILURE = 2,
  H265NAL_STATUS_DUMP_UNAVAILABLE = 3,
};

enum {
  H265NAL_DUMP_FLAG_ONE_LINE = 1u << 0,
};

int h265nal_annexb_count_nalus(
    const uint8_t* data,
    size_t len,
    size_t* out_count);

int h265nal_annexb_dump(
    const uint8_t* data,
    size_t len,
    uint32_t dump_flags);

#ifdef __cplusplus
}
#endif

#endif  // H265_C_API_H_
```

### Function Semantics
- `h265nal_annexb_count_nalus(...)`
  - Returns `H265NAL_STATUS_INVALID_ARGUMENT` if `out_count == NULL` or (`data == NULL` and `len > 0`).
  - Sets `*out_count` to number of Annex-B NAL units using `FindNaluIndices(...)`.
- `h265nal_annexb_dump(...)`
  - Returns `H265NAL_STATUS_INVALID_ARGUMENT` if (`data == NULL` and `len > 0`).
  - Returns `H265NAL_STATUS_DUMP_UNAVAILABLE` when built without `FDUMP_DEFINE`.
  - Otherwise parses with `ParseBitstream(...)` and writes per-NAL dump to `stdout`.
  - `H265NAL_DUMP_FLAG_ONE_LINE` maps to `indent_level = -1`; otherwise `indent_level = 0`.

### CMake Integration
- Add `h265_c_api.cc` to both `add_library(h265nal ...)` source lists in `src/CMakeLists.txt`.

## Rust Integration Design

### `crates/h265nal-sys`
- Add raw `unsafe extern "C"` declarations for both ABI functions.
- Add a minimal status/error mapping:
  - `InvalidArgument`
  - `ParseFailure`
  - `DumpUnavailable`
  - `UnknownStatus(i32)`
- Add tiny safe wrappers:
  - `count_nalus_annexb(data: &[u8]) -> Result<usize, Error>`
  - `dump_annexb_to_stdout(data: &[u8], flags: u32) -> Result<(), Error>`

### New CLI Crate: `crates/h265nal-cli`
- Add workspace member for `crates/h265nal-cli`.
- Input model:
  - positional `<input.265>`
  - optional `--dump`
- Behavior:
  - default: print `nal_units=<count>`
  - with `--dump`: print parser dump to stdout via FFI dump call
- Keep implementation intentionally minimal (manual arg parsing; no extra CLI framework dependency).

## Test Plan (nextest)

### Sys Tests
- Add integration tests in `crates/h265nal-sys/tests/`.
- Fixture: `media/pps_fdump_crash.202203.265`.
- Assertions:
  - count returns `> 0`
  - dump returns `Ok(())` in normal footprint builds

### CLI Tests
- Add integration tests in `crates/h265nal-cli/tests/` using `CARGO_BIN_EXE_h265nal-cli`.
- Assertions:
  - default mode prints `nal_units=`
  - `--dump` mode emits text containing `nal_unit {`

## Iteration Plan

### Iteration 1: Native C ABI Shim
1. Create `include/h265_c_api.h` with `extern "C"` guards and status/flag constants.
2. Implement `src/h265_c_api.cc` using:
   - `FindNaluIndices(...)` for count
   - `ParseBitstream(...)` + per-NAL `fdump(...)` for dump
3. Add `h265_c_api.cc` to `src/CMakeLists.txt` in both footprint branches.

Validation:
- `cargo build -p h265nal-sys -vv`

### Iteration 2: First Rust FFI Symbols
1. Update `crates/h265nal-sys/src/lib.rs` with extern declarations and wrappers.
2. Add sys integration tests for count and dump smoke behavior.

Validation:
- `cargo nextest run -p h265nal-sys`

### Iteration 3: Minimal CLI (`count` + `--dump`)
1. Create `crates/h265nal-cli` binary crate.
2. Implement argument handling for `<input>` and `--dump`.
3. Wire CLI to `h265nal-sys` wrappers.
4. Add CLI integration tests using `CARGO_BIN_EXE_h265nal-cli`.

Validation:
- `cargo nextest run -p h265nal-cli`

### Iteration 4: Workspace Smoke Gate
Run full minimal gate:
- `cargo build`
- `cargo nextest run -p h265nal-sys -p h265nal-cli`

## Acceptance Criteria
- C ABI shim builds into `libh265nal` and exports two symbols:
  - `h265nal_annexb_count_nalus`
  - `h265nal_annexb_dump`
- `h265nal-sys` can call count and dump through Rust wrappers.
- `h265nal-cli` supports:
  - `h265nal-cli <file>`
  - `h265nal-cli --dump <file>`
- Minimal nextest smoke tests pass for sys + cli crates.

## Risks / Watchouts
- Dump path depends on `FDUMP_DEFINE`; keep explicit `DUMP_UNAVAILABLE` return path.
- Large input can produce very large dump output; keep tests on a small fixture.
- Keep this pass Annex-B only to avoid scope expansion.

## Post-Plan Next Slice
- Add allocator-safe dump-to-buffer API (size query + caller-owned buffer).
- Add richer dump flags and selected parity with `tools/h265nal` options.
- Introduce a safe ergonomic wrapper crate over `h265nal-sys`.
