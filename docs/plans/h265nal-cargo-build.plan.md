# h265nal Rust Cargo/CMake Build Plan (Build-Only)

## Goal
Create a Cargo workspace and a new sys crate under `crates/` that builds the existing CMake `h265nal` native library via the Rust `cmake` crate.

This phase only covers build plumbing. Rust FFI function wrapping and safe APIs are intentionally deferred.

## Status
- Iteration 1: completed
- Iteration 2: completed
- Iteration 3: completed
- Build-only phase: complete (validated with repeat `cargo build -p h265nal-sys -vv`)

## Constraints and Assumptions
- Keep the upstream/native CMake project as the source of truth for compilation.
- Avoid introducing API bindings in this phase.
- Build must work via `cargo build` from repo root.
- Tests and fuzzing should be disabled for the sys crate build path unless explicitly requested.
- No CMake `install()` rules are currently defined, so link path discovery must be defensive.

## Decision Log (Current Defaults)
- Use a `-sys` crate first (`h265nal-sys`) and defer high-level wrapper crate.
- Build only CMake target `h265nal` (not tools/tests/fuzz targets).
- Prefer `build_target("h265nal")` over adding CMake `install()` rules in this phase.
- Keep build profile selection delegated to `cmake` crate defaults unless a target-specific issue appears.

## Deliverables
- Workspace `Cargo.toml` at repo root.
- New crate `crates/h265nal-sys/` with:
  - `Cargo.toml`
  - `build.rs`
  - `src/lib.rs`
- Cargo build integration that:
  - runs CMake configure/build through `cmake::Config`
  - builds target `h265nal`
  - links the produced native static library into the Rust crate

## Iteration Plan

### Iteration 1: Workspace + Minimal Sys Crate Skeleton
1. Add root `Cargo.toml` workspace with `resolver = "2"`.
2. Add `crates/h265nal-sys/Cargo.toml`:
   - `name = "h265nal-sys"`
   - `links = "h265nal"`
   - `build = "build.rs"`
   - `[build-dependencies] cmake = "0.1"`
3. Add placeholder `src/lib.rs` documenting build-only status.

Validation:
- `cargo metadata` succeeds.
- `cargo check -p h265nal-sys` reaches build script stage.

### Iteration 2: Native Build Wiring in `build.rs`
1. Use `cmake::Config::new("../..")` from crate dir (or absolute from `CARGO_MANIFEST_DIR`).
2. Set CMake defines:
   - `BUILD_TESTS=OFF`
   - `BUILD_CLANG_FUZZER=OFF`
   - optionally `CMAKE_CXX_CLANG_TIDY=` for portability
3. Set `build_target("h265nal")`.
4. Run `build()` and compute likely output directories.
5. Emit:
   - `cargo:rustc-link-lib=static=h265nal`
   - `cargo:rustc-link-search=native=<resolved-path>`
6. Emit platform C++ runtime link line:
   - macOS/iOS: `cargo:rustc-link-lib=dylib=c++`
   - Linux/Android: `cargo:rustc-link-lib=dylib=stdc++`
   - MSVC: none
7. Emit `cargo:rerun-if-changed` hints for CMake/source/header trees.

Validation:
- `cargo build -p h265nal-sys -vv` succeeds.
- Build script logs show CMake configure/build invoked.

### Iteration 3: Path Robustness + Dev UX
1. Make link search path resolution robust across generators by probing common locations:
   - `${dst}/build/src`
   - `${dst}/build`
   - `${dst}/lib`
   - `${dst}/lib64`
2. Fail with actionable error if library path cannot be found.
3. Add concise build diagnostics (`cargo:warning=`) for resolved paths/target.

Validation:
- Re-run `cargo build -p h265nal-sys` twice (fresh and incremental).
- Confirm repeatable success.

## Acceptance Criteria
- `cargo build -p h265nal-sys` succeeds from repository root.
- Native `h265nal` static library is built by CMake via Cargo build script.
- Cargo receives proper native link search path and link lib directives.
- No Rust FFI function bindings are introduced yet.

## Risks
- C++ standard library link differences across targets may require target-specific branching.
- CMake output directory layout can vary by generator/platform.
- Cross-compilation may require additional CMake toolchain handoff in later iterations.

## Deferred (Next Phase)
- Add explicit C ABI shim layer for selected parser entrypoints.
- Add bindgen or handwritten extern declarations.
- Add higher-level safe Rust wrapper crate (`crates/h265nal`).
