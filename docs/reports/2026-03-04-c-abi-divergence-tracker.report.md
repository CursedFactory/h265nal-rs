# C ABI Divergence Tracker (Rust Test Ports)

Date: 2026-03-04

## Why this report exists

This report tracks every `// DIVERGENCE:` marker added in C ABI code so we can:
- keep C-side changes minimal and intentional,
- explain exactly why each divergence exists,
- remove or narrow divergences once stable upstream-equivalent surfaces exist.

## Divergence policy

- Add `// DIVERGENCE:` only when required to unlock parity-port tests that cannot be implemented with existing ABI.
- Keep divergence APIs flat and explicit, with no hidden ownership transfer.
- Prefer additive helper APIs over changing native parser behavior.
- Every new `// DIVERGENCE:` marker must be added to this report in the same slice.
- If a divergence can be replaced by an existing canonical API later, mark it as convergence candidate.

## Current divergence inventory

| Symbol / Struct | Location | Why this divergence exists | Unlocks | Convergence path |
|---|---|---|---|---|
| `h265nal_bitstream_parser_state` + create/free | `include/h265_c_api.h`, `src/h265_c_api.cc` | Stateful native parsing is required across multiple test buffers; prior ABI had no state handle lifecycle. | Group 01 Case 03 (GetSliceQpY), future stateful bitstream parity | Keep as stable opaque handle; may become canonical ABI surface. |
| `h265nal_common_more_rbsp_data(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Native helper depends on `BitBuffer` cursor position; Rust tests need explicit cursor in/out checks. | Group 02 Case 06 | Could converge to a general BitBuffer ABI module if added later. |
| `h265nal_utils_get_slice_qp_y(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | `H265Utils::GetSliceQpY` was not exposed through C ABI, blocking parity on expected QP-Y values. | Group 01 Case 03 | Keep helper or replace with broader slice/bitstream result API. |
| `h265nal_bitstream_nal_fields` + `h265nal_bitstream_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Bitstream parser parity tests required flattened per-NAL metadata (`offset`, `length`, `parsed_length`), header fields, and checksum bytes with optional shared parser state. | Group 01 Cases 02/04/05 | Keep this as the shared ParseBitstream bridge; expand only if future tests require deeper payload surfaces. |
| `h265nal_profile_info_fields` | `include/h265_c_api.h` | Rust tests require deterministic scalar field access; C++ state object layout is not ABI-safe. | Group 05 profile-tier-level cases | Could converge to accessor-based API if preferred over flattened structs. |
| `h265nal_profile_tier_level_fields` | `include/h265_c_api.h` | Same reason as above; explicit flattened output avoids exposing C++ internals. | Group 05 Cases 22/23 | Same as above. |
| `h265nal_pps_fields` | `include/h265_c_api.h` | PPS parity tests need direct field assertions; no stable C ABI field surface existed. | Group 04 Case 17 | May remain canonical for parity-oriented ABI. |
| `h265nal_pps_multilayer_extension_fields` + `h265nal_pps_multilayer_extension_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | PPS multilayer extension parser had no ABI entrypoint; parity needs only four scalar fields (`poc_reset_info_present_flag`, `pps_infer_scaling_list_flag`, `num_ref_loc_offsets`, `colour_mapping_enabled_flag`). | Group 04 Case 16 | Keep flattened shape minimal; add per-offset vectors only if future tests assert them. |
| `h265nal_pps_scc_extension_fields` + `h265nal_pps_scc_extension_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | PPS SCC extension parser had no ABI entrypoint; parity requires three scalar flags plus palette-predictor initializer vector size while preserving parse-failure behavior for malformed sample. | Group 04 Cases 18/19 | Keep scalar+size surface; export initializer payload values only if future SCC tests assert contents. |
| `h265nal_pred_weight_table_fields` + `h265nal_pred_weight_table_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Pred-weight-table parser had no ABI entrypoint; parity requires scalar denominators and two L0 flag vectors without exposing C++ vector ownership. | Group 04 Case 20, Group 05 Case 21 | Keep bounded flag arrays and sizes; add L1 or delta/offset vectors only if future tests require deeper parity. |
| `h265nal_nal_unit_fields` | `include/h265_c_api.h` | NAL tests require core header + parsed-length values and checksum bytes without exposing parser-owned objects. | Group 03 Cases 14/15 | Keep bounded checksum bytes; avoid embedding full payload trees unless future tests require it. |
| `h265nal_nal_unit_header_get_nal_unit_type(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | C++ has helper API, but ABI lacked direct equivalent used by header verification test. | Group 03 Case 13 | Likely stable long-term helper. |
| `h265nal_nal_unit_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Needed to parse NAL core fields with parser state from Rust parity tests; now also returns checksum bytes for parity checks. | Group 03 Cases 14/15 | Keep payload assertions in dedicated parser APIs (`vps_parse`, etc.) unless end-to-end NAL payload bridging becomes necessary. |
| `h265nal_configuration_box_fields` + `h265nal_configuration_box_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Configuration-box tests needed an hvcC parser entrypoint and flattened asserted scalar/array fields without exposing C++ vectors/ownership semantics. | Group 03 Case 11 | Keep focused on asserted hvcC fields; only add deeper nested data if future tests require it. |
| `h265nal_hrd_parameters_fields` + `h265nal_hrd_parameters_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | HRD parser tests needed direct parser entrypoint plus flattened scalar/vector fields and first sub-layer payload in ABI-safe form. | Group 03 Case 12 | Keep bounded vector capacities and first sub-layer surface; extend only when additional sub-layer indices are asserted. |
| `h265nal_profile_tier_level_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | No ABI entrypoint for profile-tier-level parser, blocking direct parity assertions. | Group 05 Cases 22/23 | Could split into smaller accessor APIs later if desired. |
| `h265nal_pps_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | No ABI entrypoint for PPS parser, blocking sample PPS parity test. | Group 04 Case 17 | Likely stable long-term helper. |
| `h265nal_sps_fields` | `include/h265_c_api.h` | SPS tests require scalar and derived field assertions without exposing C++ object layout. This struct now keeps only scalar SPS fields plus `st_ref_pic_set_size`; detailed RPS vectors moved to dynamic helper APIs to match C++ vector sizes. | Group 09 Cases 44/45, Group 10 SPS prework | Keep scalar SPS output here and use dedicated dynamic helper calls for nested vectors. |
| `h265nal_sps_parse(...)` + `h265nal_sps_st_ref_pic_set_count/get/vector_get(...)` + `h265nal_sps_st_ref_pic_set_fields` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Replaced bounded SPS RPS flattening with explicit two-call dynamic query/fill helpers (`out_count` query then caller-provided buffer fill), avoiding truncation semantics and staying closer to C++ `std::vector` behavior. | Group 09 Cases 44/45 | Keep this accessor-style split; avoid reintroducing fixed-cap flattening in `h265nal_sps_fields`. |
| `h265nal_slice_segment_layer_fields` | `include/h265_c_api.h` | Slice parser tests require selected header fields in a stable ABI-friendly struct. | Group 08 Cases 38/39/40 and Group 09 Case 41 | Extend only for additional asserted fields. |
| `h265nal_slice_segment_layer_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | No ABI entrypoint for `H265SliceSegmentLayerParser::ParseSliceSegmentLayer`. | Group 08 Cases 38/39/40 and Group 09 Case 41 | Could converge with richer NAL payload APIs later. |
| `h265nal_rtp_fields` | `include/h265_c_api.h` | RTP parity tests need AP/FU/single packet outputs from one ABI-safe flattened result without exposing C++ ownership-heavy RTP state objects. | Group 05 Cases 24/25, Group 06 Cases 26/27/28/29/30, Group 07 Case 31 | Keep as one bounded result struct; expand only if additional RTP assertions are ported. |
| `h265nal_rtp_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | One generic `ParseRtp` bridge unlocks all RTP-family tests while minimizing new symbols and avoiding per-parser APIs. | Group 05 Cases 24/25, Group 06 Cases 26/27/28/29/30, Group 07 Case 31 | Prefer this single entrypoint unless a future test proves parser-specific symbols are required. |
| `h265nal_bitstream_parser_state_seed_vps/sps/pps(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Native slice tests seed parser state with mock VPS/SPS/PPS; ABI had no state mutation helpers. | Group 08 slice cases requiring seeded state | Replace with canonical parser-state builder API if added upstream. |
| `h265nal_sei_message_fields` | `include/h265_c_api.h` | SEI parity tests need payload-family scalar metadata without exposing polymorphic C++ payload objects. Payload bytes were removed from this struct to eliminate fixed-cap/truncation behavior. | Group 07 Cases 33/34/35, Group 08 Cases 36/37 | Keep metadata in this struct; fetch variable payload bytes through dedicated dynamic helpers. |
| `h265nal_sei_parse(...)` + `h265nal_sei_registered_itu_t_t35_payload_get(...)` + `h265nal_sei_unregistered_payload_get(...)` + `h265nal_sei_unknown_payload_get(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | `h265nal_sei_parse` remains the scalar metadata entrypoint, while payload bytes now use explicit two-call query/fill helpers to preserve full payload lengths and avoid fixed-cap truncation semantics. | Group 07 Cases 33/34/35, Group 08 Cases 36/37 | Keep one metadata parse symbol plus payload-family helper accessors. |
| `h265nal_sps_multilayer_extension_fields` | `include/h265_c_api.h` | Multilayer extension test asserts a single scalar field requiring a stable ABI surface. | Group 09 Case 43 | Likely stable minimal flattened surface. |
| `h265nal_sps_multilayer_extension_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | No ABI entrypoint for SPS multilayer extension parser. | Group 09 Case 43 | Could remain as dedicated helper API. |
| `h265nal_scaling_list_data_fields` + `h265nal_scaling_list_data_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | Scaling-list parser state is nested/vector-heavy; parity tests need a bounded flattened shape for asserted flags and matrix size/value invariants. | Group 07 Case 32 | Keep flattened view minimal; add richer matrix export only if future cases require full coefficient vectors. |
| `h265nal_sps_3d_extension_fields` + `h265nal_sps_3d_extension_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | SPS 3D extension parser had no ABI entrypoint and includes vector fields not ABI-safe without flattening. | Group 09 Case 42 | Keep current bounded vector capacity and expand only if additional indices are asserted later. |
| `h265nal_sps_range_extension_fields` + `h265nal_sps_range_extension_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | SPS range extension test requires direct scalar assertions and had no C ABI surface. | Group 10 Case 48 | Likely stable dedicated helper API. |
| `h265nal_sps_scc_extension_fields` + `h265nal_sps_scc_extension_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | SPS SCC extension parser had no ABI entrypoint; parity requires SCC scalar fields and initializer-vector size without exposing C++ containers. | Group 10 Case 49 | Keep flattened scalar/size fields; only add initializer payload export if needed by future SCC cases. |
| `h265nal_st_ref_pic_set_fields` + `h265nal_st_ref_pic_set_parse(...)` | `include/h265_c_api.h`, `src/h265_c_api.cc` | ST-ref-pic-set parser had no ABI entrypoint; parity requires selected fields and one bounded delta vector while preserving minimal C changes. | Group 10 Case 50 | Could converge to broader SPS/st-ref accessor API if deeper RPS parity is added later. |

## 2026-03-04 minimal-symbol pass note

- This pass intentionally added **no new C API symbols**.
- It only implemented three symbols that were already declared with `// DIVERGENCE:` in `include/h265_c_api.h`:
  - `h265nal_sub_layer_hrd_parameters_parse(...)`
  - `h265nal_vps_parse(...)`
  - `h265nal_vui_parameters_parse(...)`
- Rust bindings and tests were updated to consume those existing declarations rather than extending C surface area.

## 2026-03-04 SEI payload expansion pass note

- This pass intentionally added **no new C API symbols**.
- It expanded the existing `h265nal_sei_message_fields` struct and `h265nal_sei_parse(...)` implementation to expose payload-family fields for:
  - user-data-registered ITU-T T.35,
  - alpha-channel-info,
  - mastering-display-colour-volume,
  - content-light-level-info.
- Rust `h265nal-sys` FFI/raw + safe wrappers were updated to match, and Cases 33-36 were unignored.

## 2026-03-04 RTP closure pass note

- This pass added **one new C API symbol** plus one flattened result struct:
  - `h265nal_rtp_parse(...)`
  - `h265nal_rtp_fields`
- Rationale: a single generic RTP entrypoint unlocks AP/FU/single/parser test families together with fewer divergence symbols than separate parser-specific APIs.
- Rust `h265nal-sys` now exposes `rtp_parse`/`RtpFields`, and RTP Cases 24-31 are unignored.

## 2026-03-04 non-RTP extension closure pass note

- This pass added **5 new C API symbols** and **5 flattened result structs** to close parser-extension gaps:
  - `h265nal_scaling_list_data_parse(...)` + `h265nal_scaling_list_data_fields`
  - `h265nal_sps_3d_extension_parse(...)` + `h265nal_sps_3d_extension_fields`
  - `h265nal_sps_range_extension_parse(...)` + `h265nal_sps_range_extension_fields`
  - `h265nal_sps_scc_extension_parse(...)` + `h265nal_sps_scc_extension_fields`
  - `h265nal_st_ref_pic_set_parse(...)` + `h265nal_st_ref_pic_set_fields`
- Scope was intentionally bounded to fields asserted by Cases 32, 42, 48, 49, and 50.
- Rust `h265nal-sys` wrappers and test ports were updated, and those 5 cases are now unignored/running.

## 2026-03-04 PPS/pred-weight closure pass note

- This pass added **3 new C API symbols** and **3 flattened result structs**:
  - `h265nal_pps_multilayer_extension_parse(...)` + `h265nal_pps_multilayer_extension_fields`
  - `h265nal_pps_scc_extension_parse(...)` + `h265nal_pps_scc_extension_fields`
  - `h265nal_pred_weight_table_parse(...)` + `h265nal_pred_weight_table_fields`
- Scope stayed limited to fields asserted by Cases 16/18/19/20/21 and parse-failure parity for the enormous-luma-bit-depth SCC sample.
- Rust `h265nal-sys` wrappers now expose `pps_multilayer_extension_parse`, `pps_scc_extension_parse`, and `pred_weight_table_parse`; the five target tests are unignored/running.

## 2026-03-04 final closure pass note

- This pass added **4 new C API symbols** and **3 new flattened result structs**, and expanded one existing struct:
  - `h265nal_bitstream_parse(...)` + `h265nal_bitstream_nal_fields`
  - `h265nal_configuration_box_parse(...)` + `h265nal_configuration_box_fields`
  - `h265nal_hrd_parameters_parse(...)` + `h265nal_hrd_parameters_fields`
  - expanded `h265nal_nal_unit_fields` / `h265nal_nal_unit_parse(...)` with checksum bytes
- Scope was intentionally bounded to fields asserted by Cases 02/04/05/11/12/15.
- Rust `h265nal-sys` wrappers were added for bitstream/configuration-box/hrd and updated for nal checksum bytes; all six remaining skipped target tests are now unignored.

## 2026-03-04 post-parity hardening pass note

- This pass added **no new C API symbols**.
- It expanded two existing flattened structs and their parse fill paths:
  - `h265nal_sps_fields` / `h265nal_sps_parse(...)` now include bounded per-`st_ref_pic_set` introspection arrays (prediction fields and S0/S1 vector families) for deeper SPS parity assertions.
  - `h265nal_sei_message_fields` / `h265nal_sei_parse(...)` now include bounded payload-byte exports for user-data-registered, user-data-unregistered, and unknown payload families.
- Rust `h265nal-sys` FFI/raw + safe wrappers were updated to match, and SPS/SEI parity tests were expanded to assert new fields.

## 2026-03-04 dynamic SPS/SEI payload accuracy pass note

- This pass added **6 new C API symbols** and **1 new helper struct**:
  - `h265nal_sps_st_ref_pic_set_count(...)`
  - `h265nal_sps_st_ref_pic_set_get(...)`
  - `h265nal_sps_st_ref_pic_set_vector_get(...)`
  - `h265nal_sps_st_ref_pic_set_fields`
  - `h265nal_sei_registered_itu_t_t35_payload_get(...)`
  - `h265nal_sei_unregistered_payload_get(...)`
  - `h265nal_sei_unknown_payload_get(...)`
- It removed bounded array payload export from `h265nal_sei_message_fields` and bounded RPS flattening from `h265nal_sps_fields`, replacing both with explicit size-query + fill helper calls.
- Rust wrappers now return `Vec<_>` for SPS per-set vectors and SEI payload bytes, while keeping `sps_parse`/`sei_parse` as primary scalar metadata surfaces.

## Still intentionally not added (to keep divergence bounded)

- Additional typed SEI payload-family fields beyond current dynamic-cast coverage.

These remain in `docs/plans/h265nal-rust-test-port-missing-features.md` until needed by active group slices.

## Maintenance checklist (per group)

1. Port group tests.
2. If blocked, add/update missing-feature entries.
3. If API added with `// DIVERGENCE:`, add/update row in this report.
4. Re-run `cargo nextest run -p h265nal-sys` and `cargo nextest run -p h265nal-test-ports`.
5. Update group status/run log.
