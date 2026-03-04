# h265nal Parity Work TODO

## Shared Porting Docs
- [ ] Keep `docs/test-cases/README.md` aligned with native `test/*.cc` inventory.
- [ ] Maintain one pseudo-code markdown file per native `TEST_*` case (currently 59).
- [ ] Regenerate `docs/test-cases/` when native tests change, then re-check affected parity rows.

## Group-Orchestrated Rust Test Porting (59 cases)
- [ ] Keep group orchestration plan current: `docs/plans/h265nal-rust-test-port-groups.plan.md`.
- [ ] Keep trigger prompts current: `docs/plans/h265nal-rust-test-port-trigger.prompt.md`.
- [ ] Track API gaps in: `docs/plans/h265nal-rust-test-port-missing-features.md`.
- [x] Group 01 (cases 1-5)
- [x] Group 02 (cases 6-10)
- [x] Group 03 (cases 11-15)
- [x] Group 04 (cases 16-20)
- [x] Group 05 (cases 21-25)
- [x] Group 06 (cases 26-30)
- [x] Group 07 (cases 31-35)
- [x] Group 08 (cases 36-40)
- [x] Group 09 (cases 41-45)
- [x] Group 10 (cases 46-50)
- [x] Group 11 (cases 51-55)
- [x] Group 12 (cases 56-59)

## Track A: C ABI 1:1 Coverage

### Contract + Infrastructure
- [x] Freeze ABI status codes and ownership conventions.
- [x] Add ABI version function and policy notes.
- [ ] Finalize C ABI module split by dependency order.
- [x] Add `test/c_api/` harness wiring.

### Core/Shared Test Mapping (5)
- [ ] `test/h265_common_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_common_unittest/h265commonmorerbspdatatest--run.md`
  - `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypenonvcl.md`
  - `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypeunspecified.md`
  - `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypevcl.md`
  - `docs/test-cases/h265_common_unittest/h265commontest--testisslicesegment.md`
- [ ] `test/h265_utils_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpyiframe.md`
  - `docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpyiframebroken.md`
  - `docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpypframe.md`
- [ ] `test/h265_bitstream_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testmultiplebuffers.md`
  - `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testmultiplebuffersgetsliceqpy.md`
  - `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testsamplebitstream.md`
  - `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testsamplebitstreamalt.md`
- [ ] `test/h265_nal_unit_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitheaderparsertest--verifparsenalutype.md`
  - `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testemptynalunit.md`
  - `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testsamplenalunit.md`
- [ ] `test/h265_configuration_box_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_configuration_box_parser_unittest/h265configurationboxparsertest--testsampleconfigurationbox.md`

### Parameter Set + Syntax Mapping (10)
- [ ] `test/h265_profile_tier_level_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_profile_tier_level_parser_unittest/h265profiletierlevelparsertest--test022018standard.md`
  - `docs/test-cases/h265_profile_tier_level_parser_unittest/h265profiletierlevelparsertest--testsamplevalue.md`
- [ ] `test/h265_vps_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_vps_parser_unittest/h265vpsparsertest--testsamplevps.md`
  - `docs/test-cases/h265_vps_parser_unittest/h265vpsparsertest--testsamplevps2.md`
- [ ] `test/h265_sps_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testcomplexstrefpicset.md`
  - `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testsamplesps.md`
  - `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testspsbadheight.md`
  - `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testspsbadwidth.md`
- [ ] `test/h265_pps_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_pps_parser_unittest/h265ppsparsertest--testsamplepps.md`
- [ ] `test/h265_vui_parameters_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters.md`
  - `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters2.md`
  - `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparametersnvenc.md`
- [ ] `test/h265_hrd_parameters_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_hrd_parameters_parser_unittest/h265hrdparametersparsertest--testsamplehrdparameters.md`
- [ ] `test/h265_sub_layer_hrd_parameters_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_sub_layer_hrd_parameters_parser_unittest/h265sublayerhrdparametersparsertest--testsamplesublayerhrdparameters.md`
- [ ] `test/h265_pred_weight_table_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_pred_weight_table_parser_unittest/h265predweighttableparsertest--testsamplepredweighttable1.md`
  - `docs/test-cases/h265_pred_weight_table_parser_unittest/h265predweighttableparsertest--testsamplepredweighttable2.md`
- [ ] `test/h265_st_ref_pic_set_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_st_ref_pic_set_parser_unittest/h265strefpicsetparsertest--testsamplestrefpicset.md`
- [ ] `test/h265_scaling_list_data_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_scaling_list_data_parser_unittest/h265scalinglistdataparsertest--testsamplescalinglistdata01.md`

### Extension Mapping (6)
- [ ] `test/h265_sps_multilayer_extension_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_sps_multilayer_extension_parser_unittest/h265spsmultilayerextensionparsertest--testsamplespsmultilayerextension.md`
- [ ] `test/h265_sps_3d_extension_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_sps_3d_extension_parser_unittest/h265sps3dextensionparsertest--testsamplesps3dextension.md`
- [ ] `test/h265_sps_range_extension_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_sps_range_extension_parser_unittest/h265spsrangeextensionparsertest--testsamplespsrangeextension.md`
- [ ] `test/h265_sps_scc_extension_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_sps_scc_extension_parser_unittest/h265spssccextensionparsertest--testsamplespssccextension.md`
- [ ] `test/h265_pps_multilayer_extension_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_pps_multilayer_extension_parser_unittest/h265ppsmultilayerextensionparsertest--testsampleppsmultilayerextension.md`
- [ ] `test/h265_pps_scc_extension_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testenormouslumabitdepth.md`
  - `docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testsampleppssccextension.md`

### Payload Mapping (3)
- [ ] `test/h265_aud_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_aud_parser_unittest/h265audparsertest--testsampleaud.md`
- [ ] `test/h265_sei_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testalphachannelinfosei.md`
  - `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testcontentlightlevelinfosei.md`
  - `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testmasteringdisplaycolourvolumesei.md`
  - `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataregistereditutt35sei.md`
  - `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataunregisteredsei.md`
- [ ] `test/h265_slice_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice.md`
  - `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice2.md`
  - `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsamplesliceppsid15.md`
  - `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testslicecontainsshorttermrefpicset.md`

### RTP Mapping (4)
- [ ] `test/h265_rtp_single_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testmultiplertppackets.md`
  - `docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testsamplevps.md`
- [ ] `test/h265_rtp_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsampleapandfu.md`
  - `docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsamplesingle.md`
- [ ] `test/h265_rtp_ap_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_rtp_ap_parser_unittest/h265rtpapparsertest--testsampleaud.md`
- [ ] `test/h265_rtp_fu_parser_unittest.cc`
  - Case docs:
  - `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsampleend.md`
  - `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplemiddle.md`
  - `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplestart.md`

### Track A Gates
- [ ] `cargo nextest run -p h265nal-sys`
- [ ] `ctest --test-dir build --output-on-failure`

## Track B: Rust CLI MVP + 1:1

### CLI Architecture
- [x] Create `crates/h265nal-cli` and workspace wiring.
- [x] Implement clap args + serde runtime config model.
- [x] Implement config normalization layer (derived rule parity).
- [ ] Implement renderers: `c`, `json`, `pretty`.

### Native Flag Parity
- [ ] Input/output flags
- [ ] Mode flags (`dump-all`, `dump-length`, `nalu-length-bytes`, `fps`)
- [ ] Toggle flags (`as-one-line`, `add-*`)
- [x] Debug/info flags (`debug`, `quiet`, `help`, `version`)
- [x] Derived behavior parity (`add_contents` and `dump_length` implications)

### `c` Mode 30-Scenario Matrix
- [ ] Base parse scenarios (6)
- [ ] Framing scenarios (5)
- [ ] Derived behavior scenarios (4)
- [ ] Layout scenarios (2)
- [ ] Field toggle scenarios (7)
- [ ] IO sink scenarios (3)
- [x] Info/error scenarios (3)

### Diff Harness
- [ ] Compare Rust `--output-format c` vs native `build/tools/h265nal` byte-for-byte.
- [ ] Emit unified diff on mismatch.
- [ ] Gate merges on parity scenario failures.

### Track B Gates
- [x] `cargo nextest run -p h265nal-cli`
- [x] `cargo nextest run -p h265nal-sys -p h265nal-cli`

## Track C: Rust Native Suite Wrapper (Out-of-Band)

### Wrapper Crate Foundation
- [x] Add `crates/h265nal-native-suite` and workspace wiring.
- [x] Keep `h265nal-native-suite` out of workspace `default-members`.
- [x] Parse `docs/test-cases/README.md` index into case inventory.
- [x] Verify native `test/*_unittest.cc` inventory aligns with case index.

### Wrapper Coverage State
- [x] Provide Rust wrapper entries for all indexed native cases.
- [x] Track per-case wrapper status (`inventory-only` or docker-scenario proxy).
- [ ] Expand wrappers from inventory/proxy to executable per-case parity mappings.

### Staged Docker Verification
- [x] Add stage docker checks via `crates/h265nal-parity` scenarios.
- [x] Verify baseline/local comparisons for:
  - `dump_one_line`
  - `dump_multiline`
  - `dump_crash_fixture`
- [ ] Grow stage scenarios beyond MVP and map them to wrapper cases.

### Track C Gates
- [x] `cargo nextest run -p h265nal-native-suite`
- [x] `H265NAL_PARITY_RUN_DOCKER_TESTS=1 cargo nextest run -p h265nal-native-suite`

## Final Integration Gate
- [ ] `./scripts/build.sh.ts`
- [ ] `cargo build`
- [ ] `cargo nextest run -p h265nal-sys -p h265nal-cli`
