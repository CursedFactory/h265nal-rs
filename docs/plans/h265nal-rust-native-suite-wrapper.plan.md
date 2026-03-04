# h265nal Rust Native Suite Wrapper Plan

## Goal
Create an out-of-band Rust crate that wraps the native `test/*.cc` inventory so we can run a Rust-owned verification suite with `cargo nextest` while preserving thin-wrapper parity goals.

This crate must:
1. track all native parser/unit test families,
2. keep case-doc coverage aligned with `docs/test-cases/README.md`,
3. run staged Docker baseline-vs-local comparisons during migration,
4. stay outside default/main crate execution paths.

## Status
- Phase 0: complete (crate scaffold + workspace wiring + inventory checks + staged docker checks).
- Phase 1: in progress (inventory wrappers exist; executable case-level parity mapping still partial).
- Phase 2: pending.
- Phase 3: pending.

## Scope
- Crate path: `crates/h265nal-native-suite`
- Primary runtime: `cargo nextest run -p h265nal-native-suite`
- Sources of truth:
  - native tests: `test/*_unittest.cc`
  - porting docs: `docs/test-cases/README.md` + `docs/test-cases/**`
  - docker parity harness: `crates/h265nal-parity`

## Design Constraints
- Keep wrapper logic thin and data-driven.
- Prefer inventory validation before behavior assertions.
- Treat docker scenario mappings as explicit and auditable (no hidden defaults).
- Keep the crate out of workspace default members to avoid inflating main dev loops.

## Module Layout
- `crates/h265nal-native-suite/src/lib.rs`
- `crates/h265nal-native-suite/src/inventory.rs`
- `crates/h265nal-native-suite/src/suite.rs`
- `crates/h265nal-native-suite/tests/native_inventory.rs`
- `crates/h265nal-native-suite/tests/native_wrappers.rs`
- `crates/h265nal-native-suite/tests/native_file_suite.rs`
- `crates/h265nal-native-suite/tests/docker_stage.rs`

## Phase Plan

### Phase 0: Scaffold + Coverage Guardrails
1. Add crate and workspace wiring.
2. Parse native case inventory from `docs/test-cases/README.md`.
3. Verify docs index count and native file count.
4. Add staged docker compare tests that call `crates/h265nal-parity` scenarios.

Gate:
- `cargo nextest run -p h265nal-native-suite`
- `H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo nextest run -p h265nal-native-suite`

### Phase 1: Case Wrapper Expansion
1. Ensure every indexed case has a `CaseWrapper` entry.
2. Track per-case status (`inventory-only` vs docker-mapped).
3. Add deterministic notes for unmapped cases.

Gate:
- `cargo nextest run -p h265nal-native-suite -E "test(native_wrappers::*)"`

### Phase 2: Executable Family Parity Mapping
1. Add executable per-family mappings from wrapper cases to parity scenarios.
2. Introduce additional scenarios in `crates/h265nal-parity` where needed.
3. Promote high-value families first (core/shared -> parameter sets -> payload -> RTP).

Gate:
- `H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo nextest run -p h265nal-native-suite`

### Phase 3: Completion Lock
1. Eliminate `inventory-only` statuses for targeted parity families.
2. Keep `docs/plans/h265nal-parity-matrix.todo.md` synced with wrapper coverage.
3. Require docker A/B parity evidence in every report slice.

Gate:
- `H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo nextest run -p h265nal-native-suite`
- `H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo nextest run -p h265nal-parity`

## Deliverables
- Out-of-band Rust wrapper suite for native test inventory.
- Repeatable `cargo nextest` entrypoint for migration progress.
- Staged docker parity checks integrated into suite passes.
