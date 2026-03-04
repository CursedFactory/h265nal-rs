# h265nal Rust CLI MVP + 1:1 Parity Plan

## Goal
Ship a Rust-first CLI that:
1. reaches MVP quickly,
2. then reaches behavior parity with `tools/h265nal.cc`,
3. supports output formats `json`, `pretty`, and `c`.

`--output-format c` is compatibility mode and must match native output byte-for-byte in parity scenarios.

## Status
- Phase 0: complete (`clap` args, serde runtime config model, normalization layer).
- Phase 1: complete (`c` baseline path active via `h265nal-sys` with smoke + info/error tests).
- Phase 2: in progress (flag parity surface wired; behavior parity expansion pending).
- Phase 3: baseline complete (count-mode `json` and `pretty` renderers), formatting/parity polish pending.
- Phase 4: pending.

## Product Targets
- CLI crate: `crates/h265nal-cli`
- Parser backend: `h265nal-sys` C ABI bindings
- Argument parsing: `clap` derive
- Runtime config model: serde-serializable structs
- Output mode flag: `--output-format <json|pretty|c>`

## Design Constraints
- Keep modules small and readable.
- Prefer one primary struct per file.
- Keep comments concise and intent-focused.
- Minimize defensive branching after config normalization.

## Module Layout
- `crates/h265nal-cli/src/main.rs`
- `crates/h265nal-cli/src/cli/mod.rs`
- `crates/h265nal-cli/src/cli/args.rs`
- `crates/h265nal-cli/src/cli/output_format.rs`
- `crates/h265nal-cli/src/config/mod.rs`
- `crates/h265nal-cli/src/config/runtime_options.rs`
- `crates/h265nal-cli/src/config/normalize.rs`
- `crates/h265nal-cli/src/app/mod.rs`
- `crates/h265nal-cli/src/app/run_parse.rs`
- `crates/h265nal-cli/src/render/mod.rs`
- `crates/h265nal-cli/src/render/c_output.rs`
- `crates/h265nal-cli/src/render/json_output.rs`
- `crates/h265nal-cli/src/render/pretty_output.rs`
- `crates/h265nal-cli/src/ffi/mod.rs`
- `crates/h265nal-cli/src/ffi/parser.rs`

## Option Parity Scope

### Input/Output
- `-i, --infile`
- `-o, --outfile`
- `--hvcc-file`

### Modes
- `--dump-all`
- `--dump-length`
- `--nalu-length-bytes`
- `--frames-per-second`

### Toggle Flags
- `--as-one-line` / `--no-as-one-line`
- `--add-offset` / `--no-add-offset`
- `--add-length` / `--no-add-length`
- `--add-parsed-length` / `--no-add-parsed-length`
- `--add-checksum` / `--no-add-checksum`
- `--add-resolution` / `--no-add-resolution`
- `--add-contents` / `--no-add-contents`

### Debug/Info
- `-d, --debug`
- `--quiet`
- `--version`
- `-h, --help`

## Native-Compatibility Normalization Rules
- `add_contents => add_offset=true && add_length=true`
- `dump_length => add_length=true`
- require at least one source: `infile || hvcc_file`
- preserve behavior for `nalu_length_bytes` values used by native path (`-1`, `0`, `>0`)

### Clap Toggle Modeling
- Model `--foo` and `--no-foo` as separate flags in clap args.
- Resolve precedence in normalization (`RuntimeOptions`) for deterministic behavior.
- Treat the normalization layer as the single source of truth for derived flag rules.

## Output Strategy

### Shared Parse Pipeline
- One parse pipeline and one normalized runtime options struct.
- Renderer selection occurs after parse.
- `json` and `pretty` share the same serde-friendly intermediate model.

### `c` Output Mode
- Dedicated compatibility renderer path.
- Treated as strict parity contract.
- Validated with string diff tests against native `build/tools/h265nal`.

### `json` Output Mode
- Stable structural schema for machine usage.
- Avoid exposing unstable internal field names when not required.

### `pretty` Output Mode
- Human-focused readable output with color.
- No byte-level parity requirement with native output.

## Porting Input References
- Use `docs/test-cases/` as the parser-behavior reference layer while building Rust parity tests.
- `docs/test-cases/README.md` maps native `TEST_*` cases to pseudo-code docs.
- Reuse these case docs to seed CLI parity scenarios and fixture selection where parser behavior is shared.
- Use the direct per-row links in `docs/plans/h265nal-parity-matrix.todo.md` to trace each source test file to exact case-doc markdown files.
- Keep docs and parity tests in sync when upstream native tests are added or changed.

## Phase Plan

### Phase 0: Skeleton + Config Model
1. Create `crates/h265nal-cli` and workspace wiring.
2. Add clap args structs and serde runtime config structs.
3. Add explicit normalization layer for derived behavior rules.

Gate:
- `cargo run -p h265nal-cli -- --help`

### Phase 1: MVP Parse + `c` Format Baseline
1. Wire basic parse and dump calls via `h265nal-sys`.
2. Implement default run path and `--output-format c`.
3. Add smoke integration tests for success/error flows.

Gate:
- `cargo nextest run -p h265nal-cli`

### Phase 2: Full Native Flag Parity
1. Implement all native flags and semantics.
2. Validate normalized behavior rules against native tool behavior.
3. Add hvcc + bitstream combined path support.

Gate:
- `cargo nextest run -p h265nal-cli -E "test(flag_*)"`

### Phase 3: JSON + Pretty
1. Implement `json` renderer with stable structure checks.
2. Implement `pretty` renderer with readable sections and color.
3. Keep renderer logic isolated from parsing and option normalization.

Gate:
- `cargo nextest run -p h265nal-cli -E "test(json_*) + test(pretty_*)"`

### Phase 4: Strict `c` Parity Lock
1. Add scenario matrix and run native/Rust side-by-side for each case.
2. Compare output as bytes (optionally normalize line endings).
3. Block merge on any parity mismatch in `c` mode.

Gate:
- `cargo nextest run -p h265nal-cli -E "test(parity_*)"`

## `c` Mode Parity Matrix (30 Scenarios)

### Base Parse (6)
- 3 input source combinations x 2 dump modes.

### Framing Cases (5)
- `--nalu-length-bytes` in `{-1,0,1,2,4}` with stable input.

### Derived Behavior Cases (4)
- `--add-contents`
- `--dump-length`
- `--dump-length --no-add-length`
- `--add-contents --no-add-offset`

### Layout Cases (2)
- `--as-one-line`
- `--no-as-one-line`

### Field Toggle Cases (7)
- One scenario for each `--add-*` flag.

### IO Sink Cases (3)
- `-o <file>`
- `-o -`
- stdin source (`-i -`) where applicable.

### Info/Error Cases (3)
- `--help`
- `--version`
- missing-input error path.

## Test Strategy

### Integration Harness
- Use `CARGO_BIN_EXE_h265nal-cli` for Rust CLI binary resolution.
- Use `assert_cmd` for invocation and exit/output assertions.
- Optionally use `trycmd` for fixture-driven command snapshots.

### Parity Diff Harness
- Run native and Rust CLI with equivalent args.
- Compare stdout as bytes for `c` mode parity.
- On mismatch, emit unified diff for diagnosis (`similar-asserts` or equivalent).

### JSON/Pretty Validation
- JSON: parse to `serde_json::Value`, assert structural invariants.
- Pretty: snapshot-like textual assertions for key sections/markers.

### Commands
- `cargo nextest run -p h265nal-cli`
- `cargo nextest run -p h265nal-sys -p h265nal-cli`

### Porting-Doc Sync Gate
- Keep CLI parity scenario docs cross-referenced to `docs/test-cases/` entries.
- If native parser tests change, refresh `docs/test-cases/` and revisit affected CLI parity scenarios.

## Known Compatibility Quirks to Preserve
- Derived option mutation order affects output.
- CSV output in dump-length mode has strict header/row structure.
- Whitespace and newline details in C output are parity-sensitive.
- Behavior under compile flags (`FDUMP_DEFINE`, `RTP_DEFINE`) can change availability.

## Deliverables
- Modular clap+serde Rust CLI crate.
- `json`, `pretty`, and strict-parity `c` output modes.
- 30-scenario `c` parity matrix with string diff test coverage.
