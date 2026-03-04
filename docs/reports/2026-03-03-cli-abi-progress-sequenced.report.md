# Report: Sequenced Progress (CLI + C ABI + Renderers + Docker Parity)

## Metadata
- Date: 2026-03-03
- Scope sequence: 1) CLI parity foundation, 2) C ABI Phase 0 scaffold, 3) renderer expansion
- Status: completed for this slice

## What Landed

### 1) CLI parity foundation (Track B)
- Expanded CLI flag surface in `crates/h265nal-cli/src/cli/args.rs`.
- Added runtime model coverage in `crates/h265nal-cli/src/config/runtime_options.rs`.
- Added normalization precedence and derived rules in `crates/h265nal-cli/src/config/normalize.rs`:
  - `add_contents => add_offset=true && add_length=true`
  - `dump_length => add_length=true`
  - explicit toggle precedence with `--no-*` winning over `--*`
- Added info/error integration tests in `crates/h265nal-cli/tests/info_error.rs`.

### 2) C ABI Phase 0 scaffold (Track A)
- Added ABI version constant + symbol to C API:
  - `H265NAL_C_ABI_VERSION`
  - `h265nal_abi_version()`
- Files: `include/h265_c_api.h`, `src/h265_c_api.cc`.
- Extended Rust FFI surface for ABI version in `crates/h265nal-sys/src/lib.rs` and smoke test in `crates/h265nal-sys/tests/smoke.rs`.
- Added initial C harness wiring:
  - `test/c_api/CMakeLists.txt`
  - `test/c_api/h265_c_api_smoke_test.c`
  - `test/CMakeLists.txt` includes `add_subdirectory(c_api)`.
- Added contract notes doc: `docs/plans/h265nal-c-abi-contract.md`.

### 3) Renderer expansion
- Implemented working `json` count renderer in `crates/h265nal-cli/src/render/json_output.rs`.
- Implemented working `pretty` count renderer in `crates/h265nal-cli/src/render/pretty_output.rs`.
- Routed both via app flow in `crates/h265nal-cli/src/app/run_parse.rs`.
- Added renderer integration tests in `crates/h265nal-cli/tests/renderers.rs`.
- Current policy: `--dump-all` remains `c`-only; non-`c` modes return a deterministic error.

## Docker Parity Context (Now Available)
- Docker parity infrastructure exists in this branch:
  - `docker/compose.yml`
  - `docker/scripts/*`
  - `crates/h265nal-parity/*`
- MVP parity scenarios are present and runnable.

## Repository Hygiene
- Moved root-level helper/check Python scripts into `scripts/helpers/python/` to reduce repository-root noise.

## Validation Run Summary
- `cargo nextest run -p h265nal-sys -p h265nal-cli` -> pass (17/17)
- `cmake --build build --target h265_c_api_smoke_test` -> pass
- `ctest --test-dir build -R h265_c_api_smoke_test --output-on-failure` -> pass
- `H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo test -p h265nal-parity --test parity_mvp -- --nocapture` -> pass (3/3)

## Plan/TODO Updates
- `docs/plans/h265nal-rust-cli-mvp-1to1.plan.md` status updated for completed/in-progress phases.
- `docs/plans/h265nal-parity-matrix.todo.md` updated:
  - Track A contract + ABI version + c_api harness items checked.
  - Track B debug/info + derived-behavior + info/error matrix/gates checked.

## Remaining Gaps (Next Slice)
- Complete remaining native flag parity (input/output, mode semantics, IO sinks).
- Expand `c` parity scenario coverage beyond current tested subset.
- Add strict side-by-side diff gating between local and baseline for broader matrix buckets.

## Continuation Prompt
Use this prompt in the next session to continue with context and goals:

"Continue from `docs/reports/2026-03-03-cli-abi-progress-sequenced.report.md`. Keep Docker parity in scope and use `crates/h265nal-parity` for end-to-end verification. Prioritize: (1) finish remaining Track B native flag parity semantics, (2) expand `c` output parity matrix scenarios with strict byte diff checks, (3) map new scenario coverage back to `docs/plans/h265nal-parity-matrix.todo.md`, and (4) keep Track A C ABI contract stable while adding parity-focused tests." 
