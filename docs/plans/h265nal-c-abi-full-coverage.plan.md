# h265nal Full C ABI 1:1 Coverage Plan

## Goal
Deliver a complete, stable C ABI for the existing C++ parser library with 1:1 parity coverage against all native parser unit tests under `test/*.cc`.

Current native scope is 28 parser/unit test files. The C ABI work is complete only when each file has explicit parity mapping and passing validation.

## Success Criteria
- Every parser/component covered by native `test/*.cc` has corresponding C ABI entrypoints.
- Every native unit-test file has at least one mapped parity case in C ABI tests and/or Rust FFI tests.
- `cargo nextest run -p h265nal-sys` passes parity checks.
- Native `ctest` remains green.
- Coverage checklist in `docs/plans/h265nal-parity-matrix.todo.md` has no unchecked C ABI items.
- Porting notes in `docs/test-cases/` stay aligned with native tests (currently 59 case docs from 28 source files).

## Scope
- Stable C ABI module surfaces for all parser families validated today.
- Opaque-handle lifecycle APIs for all stateful parse flows.
- Error/status model, ownership model, and ABI versioning policy.
- C/C++ ABI tests and Rust FFI parity tests.

## Porting Reference Artifacts
- Generated pseudo-code docs live under `docs/test-cases/`.
- `docs/test-cases/README.md` is the index across all native `TEST_*` cases.
- Each native case has one markdown file with intent, inputs, high-level checks, and pseudocode.
- `docs/plans/h265nal-parity-matrix.todo.md` now includes direct case-doc links for each native `test/*.cc` row.
- ABI contract freeze notes live in `docs/plans/h265nal-c-abi-contract.md`.
- Use these docs as the first translation layer when implementing C ABI parity tests in other languages.

## Non-Goals
- New parser features not already in C++.
- Ergonomic high-level Rust wrapper API design (beyond parity verification helpers).

## ABI Contract Rules
- Use `extern "C"` and C-compatible types only at ABI boundaries.
- Use opaque handles for all complex or mutable parser state.
- Use explicit create/free APIs for each handle type.
- Return integer status codes (`0` success, non-zero error classes).
- Never transfer ownership implicitly.
- Separate parsing APIs from rendering/output APIs.
- Keep ABI namespaced as `h265nal_*`.

## ABI Versioning Policy
- Add an ABI version query at startup (`h265nal_abi_version()`).
- Additive changes only within major line.
- No signature changes for existing symbols.
- Document deprecations before removals.

## Module Layout
- `include/h265_c_api_common.h`
- `include/h265_c_api_nal.h`
- `include/h265_c_api_bitstream.h`
- `include/h265_c_api_vps_sps_pps.h`
- `include/h265_c_api_slice.h`
- `include/h265_c_api_sei.h`
- `include/h265_c_api_configuration_box.h`
- `include/h265_c_api_rtp.h`
- `src/h265_c_api_common.cc`
- `src/h265_c_api_nal.cc`
- `src/h265_c_api_bitstream.cc`
- `src/h265_c_api_vps_sps_pps.cc`
- `src/h265_c_api_slice.cc`
- `src/h265_c_api_sei.cc`
- `src/h265_c_api_configuration_box.cc`
- `src/h265_c_api_rtp.cc`

## Dependency-Ordered Sequencing
Implement in dependency order (not alphabetical):
1. `common`
2. `nal` (headers, payload, type)
3. `bitstream` (Annex-B and explicit length)
4. `vps/sps/pps` + syntax subparsers
5. `slice` (state dependent)
6. `sei` and `aud`
7. `configuration_box` (hvcc + state)
8. `rtp` (feature-gated)

## Native Test Parity Inventory (28 Files)

### Core/Shared
- `test/h265_common_unittest.cc`
- `test/h265_utils_unittest.cc`
- `test/h265_bitstream_parser_unittest.cc`
- `test/h265_nal_unit_parser_unittest.cc`
- `test/h265_configuration_box_parser_unittest.cc`

### Parameter Set + Syntax
- `test/h265_profile_tier_level_parser_unittest.cc`
- `test/h265_vps_parser_unittest.cc`
- `test/h265_sps_parser_unittest.cc`
- `test/h265_pps_parser_unittest.cc`
- `test/h265_vui_parameters_parser_unittest.cc`
- `test/h265_hrd_parameters_parser_unittest.cc`
- `test/h265_sub_layer_hrd_parameters_parser_unittest.cc`
- `test/h265_pred_weight_table_parser_unittest.cc`
- `test/h265_st_ref_pic_set_parser_unittest.cc`
- `test/h265_scaling_list_data_parser_unittest.cc`

### Extension Families
- `test/h265_sps_multilayer_extension_parser_unittest.cc`
- `test/h265_sps_3d_extension_parser_unittest.cc`
- `test/h265_sps_range_extension_parser_unittest.cc`
- `test/h265_sps_scc_extension_parser_unittest.cc`
- `test/h265_pps_multilayer_extension_parser_unittest.cc`
- `test/h265_pps_scc_extension_parser_unittest.cc`

### Payload Families
- `test/h265_aud_parser_unittest.cc`
- `test/h265_sei_parser_unittest.cc`
- `test/h265_slice_parser_unittest.cc`

### RTP Families (Feature-Gated)
- `test/h265_rtp_single_parser_unittest.cc`
- `test/h265_rtp_parser_unittest.cc`
- `test/h265_rtp_ap_parser_unittest.cc`
- `test/h265_rtp_fu_parser_unittest.cc`

## Phase Plan

### Phase 0: Contract + Coverage Manifest
1. Finalize ABI status code enum and ownership policy doc.
2. Add C ABI version function contract.
3. Expand `docs/plans/h265nal-parity-matrix.todo.md` with per-file checkboxes.
4. Lock `docs/test-cases/` as parity reference input for this plan.

Gate:
- `cargo build`

### Phase 1: Core Stateless Surface
1. Implement `common`, `nal`, and base `bitstream` ABI modules.
2. Add C ABI tests for helper/type and stateless parse behavior.
3. Add first Rust FFI parity tests for these modules.

Gate:
- `ctest --test-dir build -R "h265_(common|utils|nal_unit|bitstream|configuration_box)_unittest" --output-on-failure`
- `cargo nextest run -p h265nal-sys`

### Phase 2: Stateful and Parameter Surface
1. Implement parser-state handle lifecycle APIs.
2. Implement VPS/SPS/PPS + syntax parser ABI wrappers.
3. Implement slice and configuration-box stateful wrappers.
4. Add parity tests mirroring state-primed native scenarios.

Gate:
- `ctest --test-dir build -R "h265_(vps|sps|pps|slice|sei|aud|profile_tier_level|vui|hrd|sub_layer_hrd|pred_weight_table|st_ref_pic_set|scaling_list_data)_.*unittest" --output-on-failure`
- `cargo nextest run -p h265nal-sys`

### Phase 3: RTP Surface
1. Implement RTP ABI wrappers under existing feature gates.
2. Add feature-gated parity tests.
3. Validate behavior in normal footprint builds.

Gate:
- `ctest --test-dir build -R "h265_rtp_.*_unittest|h265_rtp_parser_unittest" --output-on-failure`
- `cargo nextest run -p h265nal-sys`

### Phase 4: Completion Lock
1. Verify all 28 native test files are checked complete in parity matrix.
2. Run full build/test matrix.
3. Block merge on any parity gap.

Gate:
- `./scripts/build.sh.ts`
- `cargo build`
- `cargo nextest run -p h265nal-sys`

## Testing Strategy

### C ABI Tests
- Add dedicated C/C++ tests under `test/c_api/`.
- Keep fixtures mostly inline, matching native test style.
- Test status codes, ownership, and deterministic field getters.

### Rust FFI Tests
- Add parity modules in `crates/h265nal-sys/tests/`.
- Mirror representative vectors and assertions per native test family.
- Prefer deterministic assertions over broad snapshot matching.

### Coverage Enforcement
- Keep per-file completion matrix in `docs/plans/h265nal-parity-matrix.todo.md`.
- CI fails if any native test file is unmapped or unchecked.
- Keep per-case pseudo-code docs in `docs/test-cases/` synchronized with source tests.
- If native `TEST_*` cases change, regenerate `docs/test-cases/` and update matrix references.

## Known Native Compatibility Quirks to Preserve
- `FDUMP_DEFINE` / `RTP_DEFINE` compile gates affect available behavior.
- Slice parsing requires prior parser-state priming with VPS/SPS/PPS in many paths.
- Explicit-length parse path behavior should be parity-pinned before any semantic cleanup.

## Deliverables
- Full C ABI headers and sources by module.
- Full parity mapping for all 28 native parser unit test files.
- Passing native `ctest` and Rust `nextest` parity gates.
