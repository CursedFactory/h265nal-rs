# h265nal Docker Dual CLI Parity Plan

## Goal
Establish a robust, reproducible environment to verify 1:1 functional parity between the upstream C++ `h265nal` CLI and our Rust port. This will be achieved using a dual-container Docker Compose setup and a dedicated Rust parity-testing crate that invokes both CLIs and strictly diffs their outputs.

## Status
Planned

## Scope

### Docker Environment (`docker/`)
- **Baseline Container**: Directly clones, builds, and invokes the upstream C++ CLI from `https://github.com/chemag/h265nal`.
- **Baseline Purity Rule**: Baseline runs only from its own upstream clone and build cache volumes; it must not compile or execute local source files from this repo.
- **Local Container**: Builds and invokes our Rust version from this repository.
- **Passthrough Interface**: Both containers must support external CLI passthrough invocation via `docker compose run --rm -T`.
- **Shared Context**: A shared mount path `/work` will be used to provide test fixtures to both services.
- **Caching & Pinning**: Use named volumes for baseline source/build and local cargo/target caches. Pin the upstream reference using an environment variable (`UPSTREAM_REF`).

### Parity Crate (`crates/h265nal-parity`)
- A new Rust crate containing a binary and integration tests to orchestrate the parity checks.
- **Modules**:
  - Scenario mapping
  - Runner (invokes Docker containers)
  - Diff (strict bytes first, unified diff diagnostics on mismatch)
  - Normalize (handles expected environmental differences)
  - Integration tests

### Alignment
This plan directly supports the goals outlined in `h265nal-rust-cli-mvp-1to1.plan.md` by providing the validation mechanism for the `c` output mode. It will systematically execute the scenarios defined in `h265nal-parity-matrix.todo.md`.

## Proposed Directory Tree

```text
docker/
├── .env.example
├── compose.yml
├── baseline.Dockerfile
├── local.Dockerfile
└── scripts/
    ├── baseline_entry.sh
    ├── local_entry.sh
    ├── run-baseline.sh
    └── run-local.sh
crates/h265nal-parity/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── runner.rs
    ├── diff.rs
    ├── normalize.rs
    └── scenarios.rs
```

## Design Constraints & Inspiration
- **Compose Inspiration**: Follow the "manual CLI services" pattern from docs.rs (e.g., prefer reusable anchors/sections).
- **Diff Policy**: Strict byte-for-byte comparison first. If a mismatch occurs, emit a unified diff for diagnostics.
- **Execution**: Use `docker compose run --rm -T <service> <args>` to pipe inputs and outputs cleanly without TTY interference.

## Phases

### Phase 1: Docker Compose Foundation
1. Create the `docker/` directory structure and `.env.example`.
2. Write `baseline.Dockerfile` to clone (using `UPSTREAM_REF`), build, and set up the C++ CLI.
3. Write `local.Dockerfile` to build the Rust CLI using cargo caches.
4. Configure `compose.yml` with shared `/work` volumes and named caches.
5. Implement entrypoint scripts to handle CLI passthrough.

**Gate**:
- `docker compose run --rm -T baseline --version` succeeds.
- `docker compose run --rm -T local --version` succeeds.

### Phase 2: Parity Crate Skeleton
1. Initialize `crates/h265nal-parity`.
2. Implement the `runner` module to wrap `docker compose run` commands.
3. Implement the `diff` module for byte comparison and unified diff output.
4. Implement the `normalize` module for basic sanitization (if needed).

**Gate**:
- `cargo run -p h265nal-parity -- --help` succeeds.

### Phase 3: MVP Parity Rollout
1. Define the first set of scenarios in the parity crate, focusing on the current local CLI capabilities (`--dump`, `--one-line`, positional input path).
2. Wire the integration tests to run these specific scenarios through both containers.
3. Ensure the diff policy correctly identifies matches and highlights mismatches.

**Gate**:
- `cargo nextest run -p h265nal-parity -E "test(mvp_*)"` passes.

### Phase 4: Full 1:1 Matrix Expansion
1. Expand the scenario definitions in `crates/h265nal-parity` to cover the full matrix defined in `h265nal-parity-matrix.todo.md`.
2. Implement any necessary normalization rules for edge cases (e.g., specific error message formats if they intentionally differ, though strict parity is preferred).
3. Integrate the parity run into the CI pipeline.

**Gate**:
- `cargo nextest run -p h265nal-parity` passes for all scenarios.

## Risks
- **Line Endings**: CRLF vs LF differences between environments or tools.
- **Locale/TZ**: Timezone or locale differences affecting output formatting.
- **Color/TTY**: ANSI escape codes bleeding into the output if TTY is not strictly disabled (`-T` flag is crucial).
- **Buffering**: Output buffering differences causing deadlocks or incomplete reads when piping.
- **Deterministic Upstream Ref**: Ensuring `UPSTREAM_REF` is always set and points to a stable, known commit to prevent flaky tests.
- **Baseline Drift**: If upstream default branch changes, parity can shift unexpectedly; pin `UPSTREAM_REF` to a commit SHA in CI.

## Deliverables
- Fully functional `docker/` environment with `baseline` and `local` CLI services.
- `crates/h265nal-parity` crate capable of orchestrating and diffing CLI runs.
- CI integration executing the parity matrix against every PR.

## Command Snippets

### Host Invocation (Manual Testing)
```bash
# Run baseline C++ CLI
docker compose -f docker/compose.yml run --rm -T baseline -- -i /work/media/nvenc.265 --dump-all --as-one-line

# Run local Rust CLI
docker compose -f docker/compose.yml run --rm -T local -- --dump --one-line /work/media/nvenc.265
```

### CI / Local Gates
```bash
# Build containers
docker compose -f docker/compose.yml build

# Run the parity test suite
cargo nextest run -p h265nal-parity

# Run a specific parity scenario
cargo run -p h265nal-parity -- --scenario dump_one_line
```
