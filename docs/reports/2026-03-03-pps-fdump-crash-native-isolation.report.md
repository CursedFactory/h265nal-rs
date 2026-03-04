# Report: pps_fdump_crash Native Isolation

## Metadata
- Date: 2026-03-03
- Scope: isolate `media/pps_fdump_crash.202203.265` dump segfault cause
- Status: completed

## Question
- Is the crash caused by Rust FFI/wrapper logic, or by underlying native parse/dump code?

## Investigation Setup
- Rust CLI path under test: `h265nal-cli --dump <input.265>`
- Direct C ABI harness source: `docs/reports/repro_h265_c_api_dump.c`
- Native static library used by harness:
  - `target/debug/build/h265nal-sys-c7fc46a5de476cd3/out/build/src/libh265nal.a`
- Upstream native CLI built from same CMake tree:
  - `target/debug/build/h265nal-sys-c7fc46a5de476cd3/out/build/tools/h265nal`

## Commands and Results

### A) Rust CLI
- `cargo run -p h265nal-cli -- --dump media/nvenc.265` -> success
- `cargo run -p h265nal-cli -- --dump media/pps_fdump_crash.202203.265` -> exits `139` / signal 11

### B) Direct C ABI (no Rust wrapper)
- Build harness:
```bash
clang -std=c11 -Iinclude docs/reports/repro_h265_c_api_dump.c \
  target/debug/build/h265nal-sys-c7fc46a5de476cd3/out/build/src/libh265nal.a \
  -lc++ -o target/repro_h265_c_api_dump
```
- Run harness (`h265nal_annexb_dump`):
  - `target/repro_h265_c_api_dump media/nvenc.265` -> status `0`
  - `target/repro_h265_c_api_dump media/pps_fdump_crash.202203.265` -> return `-11` (SIGSEGV)

### C) Upstream native tool
- Built target:
```bash
cmake --build target/debug/build/h265nal-sys-c7fc46a5de476cd3/out/build --target h265nal-bin
```
- Run tool directly:
  - `.../out/build/tools/h265nal -i media/nvenc.265` -> `0`
  - `.../out/build/tools/h265nal -i media/pps_fdump_crash.202203.265` -> `-11` (SIGSEGV)

## Debug Evidence
- LLDB crash frame during dump:
  - `src/h265_pps_multilayer_extension_parser.cc:210`
  - `h265nal::H265PpsMultilayerExtensionParser::PpsMultilayerExtensionState::fdump`
- Observed frame state:
  - `this = 0x0` (null dereference)

## Root-Cause Hypothesis (High Confidence)
1. Parser can return null multilayer extension state for unsupported syntax:
   - `colour_mapping_enabled_flag` returns `nullptr` in `src/h265_pps_multilayer_extension_parser.cc:188`.
2. PPS parse path stores this pointer without null guard:
   - assignment in `src/h265_pps_parser.cc:303`.
3. Dump path assumes pointer validity when extension flag is set:
   - unconditional dereference in `src/h265_pps_parser.cc:528`.

## Conclusion
- Crash reproduces in:
  - Rust CLI path,
  - direct C ABI path,
  - upstream native C++ CLI path.
- Therefore this is a native library parse/dump bug, not a Rust-wrapper bug.

## Follow-on Fix Report
- `docs/reports/2026-03-03-pps-fdump-crash-hardening-fix.report.md`

## Recommended Fix Path
1. Add null guard before dumping `pps_multilayer_extension` in `src/h265_pps_parser.cc`.
2. Decide policy for unsupported extension parse branches:
   - strict fail and stop,
   - or partial parse with explicit dump-safe guards.
3. Convert ignored crash regression to normal passing test once native hardening lands.
