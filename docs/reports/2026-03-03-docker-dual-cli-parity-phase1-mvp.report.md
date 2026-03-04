# Report: Docker Dual CLI Parity (Phase 1 + MVP Harness)

## Metadata
- Date: 2026-03-03
- Scope plan: `docs/plans/h265nal-docker-dual-cli-parity.plan.md`
- Status: phase 1 complete, MVP parity harness complete

## Summary
- Added a two-service Docker Compose foundation for side-by-side CLI execution.
- Enforced a pure upstream baseline container that clones and builds `https://github.com/chemag/h265nal` (no local-source baseline build).
- Added a new Rust crate `crates/h265nal-parity` with MVP parity tests that run both containers and compare exit code/stdout/stderr.

## Changes

### Plan and docs
- `docs/plans/h265nal-docker-dual-cli-parity.plan.md`: updated baseline source to `https://github.com/chemag/h265nal` and added baseline purity rule.

### Docker foundation
- `docker/compose.yml`: added `baseline` and `local` services only; shared `/work` bind mount; named cache volumes.
- `docker/.env.example`: added `UPSTREAM_REPO_URL` and `UPSTREAM_REF` defaults.
- `docker/baseline.Dockerfile`: baseline image with git/cmake/build tools and baseline entrypoint.
- `docker/local.Dockerfile`: local Rust image with cargo/cmake/build tools and local entrypoint.
- `docker/scripts/baseline_entry.sh`: clone/fetch/checkout/build upstream and exec upstream `h265nal`.
- `docker/scripts/local_entry.sh`: build local `h265nal-cli` with locked deps and exec binary.
- `docker/scripts/run-baseline.sh`: host helper passthrough wrapper.
- `docker/scripts/run-local.sh`: host helper passthrough wrapper.

### Parity crate
- `Cargo.toml`: added workspace member `crates/h265nal-parity`.
- `crates/h265nal-parity/Cargo.toml`: new crate manifest.
- `crates/h265nal-parity/src/scenario.rs`: MVP scenario definitions and arg mapping.
- `crates/h265nal-parity/src/runner.rs`: docker compose command runner for both services.
- `crates/h265nal-parity/src/diff.rs`: strict byte mismatch detection + unified text diff diagnostics.
- `crates/h265nal-parity/src/normalize.rs`: normalization hook (identity for now).
- `crates/h265nal-parity/src/lib.rs`: compare orchestration and parity assertion logic.
- `crates/h265nal-parity/src/main.rs`: CLI entry (`compare --scenario ... [--fixture ...]`).
- `crates/h265nal-parity/tests/parity_mvp.rs`: Rust integration tests for MVP parity scenarios.

## Validation
- `bash -n docker/scripts/baseline_entry.sh docker/scripts/local_entry.sh docker/scripts/run-baseline.sh docker/scripts/run-local.sh` -> pass
- `docker compose -f docker/compose.yml config` -> pass
- `cargo test -p h265nal-parity` -> pass
- `H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo test -p h265nal-parity --test parity_mvp -- --nocapture` -> pass (3/3)
- `cargo nextest run` -> pass (18/18, including parity MVP tests)

## Issues and Notes

### Resolved
1. Baseline source target corrected from prior fork to upstream `https://github.com/chemag/h265nal`.
2. Baseline purity enforced in plan and runtime behavior (baseline clone/build from dedicated volumes only).
3. Added deterministic passthrough contract for both services using `docker compose run --rm -T`.

### Open
1. MVP coverage is limited to dump-focused scenarios; full native flag matrix is not yet implemented.
2. Compare normalization is intentionally minimal; future cases may require optional newline/color normalization policies.
3. Docker-backed tests are gated by `H265NAL_PARITY_RUN_DOCKER_TESTS=1`; CI policy still needs explicit wiring.

## Parity Readiness Estimate
- **Infrastructure readiness**: ~85%
  - Dual-service compose, upstream/local isolation, passthrough contract, and Rust harness are in place.
- **Behavior coverage readiness**: ~20%
  - MVP validates only 3 dump scenarios; full 1:1 parity requires expanding to the full scenario matrix from the parity plan.
- **Overall readiness for full 1:1 functional parity gate**: ~40%

## Next Steps
1. Expand `crates/h265nal-parity/src/scenario.rs` from MVP to full matrix (input/output, modes, toggles, info/error paths).
2. Add per-scenario policy for strict-vs-normalized comparison where needed, defaulting to strict bytes.
3. Wire docker-backed parity tests into CI with pinned `UPSTREAM_REF` commit SHA.
4. Align scenario names and checklist completion against `docs/plans/h265nal-parity-matrix.todo.md`.
5. Add report follow-up after first non-MVP expansion batch (target: 10+ scenarios).
