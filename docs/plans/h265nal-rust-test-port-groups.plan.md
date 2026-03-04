# H265NAL Rust Test Port Groups Plan

## Goal and Workflow
- Goal: split the indexed cases from `docs/test-cases/README.md` into sequential groups of 5 markdown files (final group is remainder) for Rust test-port execution.
- Workflow: parent agent assigns one group at a time to `squad_developer` (group context + plan), then `squad_developer` fans out one `squad_intern` task per markdown case in that group.
- Coverage target: 59 indexed cases in README order, across 12 groups (11 groups of 5, 1 group of 4).

## Execution Rules
- Process groups strictly sequentially: complete Group 01 before starting Group 02, etc.
- Update this file when each group starts and completes (checkbox + run log row).
- If a required Rust API/feature is missing: add a TODO short-circuit in the test, then append a missing-feature report entry to `docs/plans/h265nal-rust-test-port-missing-features.md`.

## Group Status
- [x] Group 01 (cases 1-5)
- [x] Group 02 (cases 6-10)
- [x] Group 03 (cases 11-15)
- [x] Group 04 (cases 16-20)
- [x] Group 05 (cases 21-25)
- [ ] Group 06 (cases 26-30) - in progress
- [ ] Group 07 (cases 31-35)
- [ ] Group 08 (cases 36-40)
- [ ] Group 09 (cases 41-45)
- [ ] Group 10 (cases 46-50)
- [ ] Group 11 (cases 51-55)
- [ ] Group 12 (cases 56-59)

## Group 01
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 01 | `docs/test-cases/h265_aud_parser_unittest/h265audparsertest--testsampleaud.md` | `test/h265_aud_parser_unittest.cc:21` | `Port test/h265_aud_parser_unittest.cc:21 using docs/test-cases/h265_aud_parser_unittest/h265audparsertest--testsampleaud.md` |
| 02 | `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testmultiplebuffers.md` | `test/h265_bitstream_parser_unittest.cc:289` | `Port test/h265_bitstream_parser_unittest.cc:289 using docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testmultiplebuffers.md` |
| 03 | `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testmultiplebuffersgetsliceqpy.md` | `test/h265_bitstream_parser_unittest.cc:413` | `Port test/h265_bitstream_parser_unittest.cc:413 using docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testmultiplebuffersgetsliceqpy.md` |
| 04 | `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testsamplebitstream.md` | `test/h265_bitstream_parser_unittest.cc:57` | `Port test/h265_bitstream_parser_unittest.cc:57 using docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testsamplebitstream.md` |
| 05 | `docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testsamplebitstreamalt.md` | `test/h265_bitstream_parser_unittest.cc:135` | `Port test/h265_bitstream_parser_unittest.cc:135 using docs/test-cases/h265_bitstream_parser_unittest/h265bitstreamparsertest--testsamplebitstreamalt.md` |

## Group 02
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 06 | `docs/test-cases/h265_common_unittest/h265commonmorerbspdatatest--run.md` | `test/h265_common_unittest.cc:129` | `Port test/h265_common_unittest.cc:129 using docs/test-cases/h265_common_unittest/h265commonmorerbspdatatest--run.md` |
| 07 | `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypenonvcl.md` | `test/h265_common_unittest.cc:34` | `Port test/h265_common_unittest.cc:34 using docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypenonvcl.md` |
| 08 | `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypeunspecified.md` | `test/h265_common_unittest.cc:40` | `Port test/h265_common_unittest.cc:40 using docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypeunspecified.md` |
| 09 | `docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypevcl.md` | `test/h265_common_unittest.cc:27` | `Port test/h265_common_unittest.cc:27 using docs/test-cases/h265_common_unittest/h265commontest--testisnalunittypevcl.md` |
| 10 | `docs/test-cases/h265_common_unittest/h265commontest--testisslicesegment.md` | `test/h265_common_unittest.cc:22` | `Port test/h265_common_unittest.cc:22 using docs/test-cases/h265_common_unittest/h265commontest--testisslicesegment.md` |

## Group 03
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 11 | `docs/test-cases/h265_configuration_box_parser_unittest/h265configurationboxparsertest--testsampleconfigurationbox.md` | `test/h265_configuration_box_parser_unittest.cc:21` | `Port test/h265_configuration_box_parser_unittest.cc:21 using docs/test-cases/h265_configuration_box_parser_unittest/h265configurationboxparsertest--testsampleconfigurationbox.md` |
| 12 | `docs/test-cases/h265_hrd_parameters_parser_unittest/h265hrdparametersparsertest--testsamplehrdparameters.md` | `test/h265_hrd_parameters_parser_unittest.cc:21` | `Port test/h265_hrd_parameters_parser_unittest.cc:21 using docs/test-cases/h265_hrd_parameters_parser_unittest/h265hrdparametersparsertest--testsamplehrdparameters.md` |
| 13 | `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitheaderparsertest--verifparsenalutype.md` | `test/h265_nal_unit_parser_unittest.cc:143` | `Port test/h265_nal_unit_parser_unittest.cc:143 using docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitheaderparsertest--verifparsenalutype.md` |
| 14 | `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testemptynalunit.md` | `test/h265_nal_unit_parser_unittest.cc:126` | `Port test/h265_nal_unit_parser_unittest.cc:126 using docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testemptynalunit.md` |
| 15 | `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testsamplenalunit.md` | `test/h265_nal_unit_parser_unittest.cc:22` | `Port test/h265_nal_unit_parser_unittest.cc:22 using docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testsamplenalunit.md` |

## Group 04
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 16 | `docs/test-cases/h265_pps_multilayer_extension_parser_unittest/h265ppsmultilayerextensionparsertest--testsampleppsmultilayerextension.md` | `test/h265_pps_multilayer_extension_parser_unittest.cc:21` | `Port test/h265_pps_multilayer_extension_parser_unittest.cc:21 using docs/test-cases/h265_pps_multilayer_extension_parser_unittest/h265ppsmultilayerextensionparsertest--testsampleppsmultilayerextension.md` |
| 17 | `docs/test-cases/h265_pps_parser_unittest/h265ppsparsertest--testsamplepps.md` | `test/h265_pps_parser_unittest.cc:21` | `Port test/h265_pps_parser_unittest.cc:21 using docs/test-cases/h265_pps_parser_unittest/h265ppsparsertest--testsamplepps.md` |
| 18 | `docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testenormouslumabitdepth.md` | `test/h265_pps_scc_extension_parser_unittest.cc:40` | `Port test/h265_pps_scc_extension_parser_unittest.cc:40 using docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testenormouslumabitdepth.md` |
| 19 | `docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testsampleppssccextension.md` | `test/h265_pps_scc_extension_parser_unittest.cc:21` | `Port test/h265_pps_scc_extension_parser_unittest.cc:21 using docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testsampleppssccextension.md` |
| 20 | `docs/test-cases/h265_pred_weight_table_parser_unittest/h265predweighttableparsertest--testsamplepredweighttable1.md` | `test/h265_pred_weight_table_parser_unittest.cc:21` | `Port test/h265_pred_weight_table_parser_unittest.cc:21 using docs/test-cases/h265_pred_weight_table_parser_unittest/h265predweighttableparsertest--testsamplepredweighttable1.md` |

## Group 05
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 21 | `docs/test-cases/h265_pred_weight_table_parser_unittest/h265predweighttableparsertest--testsamplepredweighttable2.md` | `test/h265_pred_weight_table_parser_unittest.cc:40` | `Port test/h265_pred_weight_table_parser_unittest.cc:40 using docs/test-cases/h265_pred_weight_table_parser_unittest/h265predweighttableparsertest--testsamplepredweighttable2.md` |
| 22 | `docs/test-cases/h265_profile_tier_level_parser_unittest/h265profiletierlevelparsertest--test022018standard.md` | `test/h265_profile_tier_level_parser_unittest.cc:69` | `Port test/h265_profile_tier_level_parser_unittest.cc:69 using docs/test-cases/h265_profile_tier_level_parser_unittest/h265profiletierlevelparsertest--test022018standard.md` |
| 23 | `docs/test-cases/h265_profile_tier_level_parser_unittest/h265profiletierlevelparsertest--testsamplevalue.md` | `test/h265_profile_tier_level_parser_unittest.cc:20` | `Port test/h265_profile_tier_level_parser_unittest.cc:20 using docs/test-cases/h265_profile_tier_level_parser_unittest/h265profiletierlevelparsertest--testsamplevalue.md` |
| 24 | `docs/test-cases/h265_rtp_ap_parser_unittest/h265rtpapparsertest--testsampleaud.md` | `test/h265_rtp_ap_parser_unittest.cc:21` | `Port test/h265_rtp_ap_parser_unittest.cc:21 using docs/test-cases/h265_rtp_ap_parser_unittest/h265rtpapparsertest--testsampleaud.md` |
| 25 | `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsampleend.md` | `test/h265_rtp_fu_parser_unittest.cc:104` | `Port test/h265_rtp_fu_parser_unittest.cc:104 using docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsampleend.md` |

## Group 06
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 26 | `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplemiddle.md` | `test/h265_rtp_fu_parser_unittest.cc:77` | `Port test/h265_rtp_fu_parser_unittest.cc:77 using docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplemiddle.md` |
| 27 | `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplestart.md` | `test/h265_rtp_fu_parser_unittest.cc:21` | `Port test/h265_rtp_fu_parser_unittest.cc:21 using docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplestart.md` |
| 28 | `docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsampleapandfu.md` | `test/h265_rtp_parser_unittest.cc:59` | `Port test/h265_rtp_parser_unittest.cc:59 using docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsampleapandfu.md` |
| 29 | `docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsamplesingle.md` | `test/h265_rtp_parser_unittest.cc:22` | `Port test/h265_rtp_parser_unittest.cc:22 using docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsamplesingle.md` |
| 30 | `docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testmultiplertppackets.md` | `test/h265_rtp_single_parser_unittest.cc:45` | `Port test/h265_rtp_single_parser_unittest.cc:45 using docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testmultiplertppackets.md` |

## Group 07
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 31 | `docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testsamplevps.md` | `test/h265_rtp_single_parser_unittest.cc:24` | `Port test/h265_rtp_single_parser_unittest.cc:24 using docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testsamplevps.md` |
| 32 | `docs/test-cases/h265_scaling_list_data_parser_unittest/h265scalinglistdataparsertest--testsamplescalinglistdata01.md` | `test/h265_scaling_list_data_parser_unittest.cc:21` | `Port test/h265_scaling_list_data_parser_unittest.cc:21 using docs/test-cases/h265_scaling_list_data_parser_unittest/h265scalinglistdataparsertest--testsamplescalinglistdata01.md` |
| 33 | `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testalphachannelinfosei.md` | `test/h265_sei_parser_unittest.cc:76` | `Port test/h265_sei_parser_unittest.cc:76 using docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testalphachannelinfosei.md` |
| 34 | `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testcontentlightlevelinfosei.md` | `test/h265_sei_parser_unittest.cc:143` | `Port test/h265_sei_parser_unittest.cc:143 using docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testcontentlightlevelinfosei.md` |
| 35 | `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testmasteringdisplaycolourvolumesei.md` | `test/h265_sei_parser_unittest.cc:96` | `Port test/h265_sei_parser_unittest.cc:96 using docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testmasteringdisplaycolourvolumesei.md` |

## Group 08
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 36 | `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataregistereditutt35sei.md` | `test/h265_sei_parser_unittest.cc:21` | `Port test/h265_sei_parser_unittest.cc:21 using docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataregistereditutt35sei.md` |
| 37 | `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataunregisteredsei.md` | `test/h265_sei_parser_unittest.cc:50` | `Port test/h265_sei_parser_unittest.cc:50 using docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataunregisteredsei.md` |
| 38 | `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice.md` | `test/h265_slice_parser_unittest.cc:26` | `Port test/h265_slice_parser_unittest.cc:26 using docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice.md` |
| 39 | `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice2.md` | `test/h265_slice_parser_unittest.cc:77` | `Port test/h265_slice_parser_unittest.cc:77 using docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice2.md` |
| 40 | `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsamplesliceppsid15.md` | `test/h265_slice_parser_unittest.cc:128` | `Port test/h265_slice_parser_unittest.cc:128 using docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsamplesliceppsid15.md` |

## Group 09
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 41 | `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testslicecontainsshorttermrefpicset.md` | `test/h265_slice_parser_unittest.cc:179` | `Port test/h265_slice_parser_unittest.cc:179 using docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testslicecontainsshorttermrefpicset.md` |
| 42 | `docs/test-cases/h265_sps_3d_extension_parser_unittest/h265sps3dextensionparsertest--testsamplesps3dextension.md` | `test/h265_sps_3d_extension_parser_unittest.cc:21` | `Port test/h265_sps_3d_extension_parser_unittest.cc:21 using docs/test-cases/h265_sps_3d_extension_parser_unittest/h265sps3dextensionparsertest--testsamplesps3dextension.md` |
| 43 | `docs/test-cases/h265_sps_multilayer_extension_parser_unittest/h265spsmultilayerextensionparsertest--testsamplespsmultilayerextension.md` | `test/h265_sps_multilayer_extension_parser_unittest.cc:21` | `Port test/h265_sps_multilayer_extension_parser_unittest.cc:21 using docs/test-cases/h265_sps_multilayer_extension_parser_unittest/h265spsmultilayerextensionparsertest--testsamplespsmultilayerextension.md` |
| 44 | `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testcomplexstrefpicset.md` | `test/h265_sps_parser_unittest.cc:191` | `Port test/h265_sps_parser_unittest.cc:191 using docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testcomplexstrefpicset.md` |
| 45 | `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testsamplesps.md` | `test/h265_sps_parser_unittest.cc:21` | `Port test/h265_sps_parser_unittest.cc:21 using docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testsamplesps.md` |

## Group 10
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 46 | `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testspsbadheight.md` | `test/h265_sps_parser_unittest.cc:176` | `Port test/h265_sps_parser_unittest.cc:176 using docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testspsbadheight.md` |
| 47 | `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testspsbadwidth.md` | `test/h265_sps_parser_unittest.cc:161` | `Port test/h265_sps_parser_unittest.cc:161 using docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testspsbadwidth.md` |
| 48 | `docs/test-cases/h265_sps_range_extension_parser_unittest/h265spsrangeextensionparsertest--testsamplespsrangeextension.md` | `test/h265_sps_range_extension_parser_unittest.cc:21` | `Port test/h265_sps_range_extension_parser_unittest.cc:21 using docs/test-cases/h265_sps_range_extension_parser_unittest/h265spsrangeextensionparsertest--testsamplespsrangeextension.md` |
| 49 | `docs/test-cases/h265_sps_scc_extension_parser_unittest/h265spssccextensionparsertest--testsamplespssccextension.md` | `test/h265_sps_scc_extension_parser_unittest.cc:21` | `Port test/h265_sps_scc_extension_parser_unittest.cc:21 using docs/test-cases/h265_sps_scc_extension_parser_unittest/h265spssccextensionparsertest--testsamplespssccextension.md` |
| 50 | `docs/test-cases/h265_st_ref_pic_set_parser_unittest/h265strefpicsetparsertest--testsamplestrefpicset.md` | `test/h265_st_ref_pic_set_parser_unittest.cc:22` | `Port test/h265_st_ref_pic_set_parser_unittest.cc:22 using docs/test-cases/h265_st_ref_pic_set_parser_unittest/h265strefpicsetparsertest--testsamplestrefpicset.md` |

## Group 11
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 51 | `docs/test-cases/h265_sub_layer_hrd_parameters_parser_unittest/h265sublayerhrdparametersparsertest--testsamplesublayerhrdparameters.md` | `test/h265_sub_layer_hrd_parameters_parser_unittest.cc:21` | `Port test/h265_sub_layer_hrd_parameters_parser_unittest.cc:21 using docs/test-cases/h265_sub_layer_hrd_parameters_parser_unittest/h265sublayerhrdparametersparsertest--testsamplesublayerhrdparameters.md` |
| 52 | `docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpyiframe.md` | `test/h265_utils_unittest.cc:25` | `Port test/h265_utils_unittest.cc:25 using docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpyiframe.md` |
| 53 | `docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpyiframebroken.md` | `test/h265_utils_unittest.cc:74` | `Port test/h265_utils_unittest.cc:74 using docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpyiframebroken.md` |
| 54 | `docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpypframe.md` | `test/h265_utils_unittest.cc:103` | `Port test/h265_utils_unittest.cc:103 using docs/test-cases/h265_utils_unittest/h265utilstest--testgetsliceqpypframe.md` |
| 55 | `docs/test-cases/h265_vps_parser_unittest/h265vpsparsertest--testsamplevps.md` | `test/h265_vps_parser_unittest.cc:21` | `Port test/h265_vps_parser_unittest.cc:21 using docs/test-cases/h265_vps_parser_unittest/h265vpsparsertest--testsamplevps.md` |

## Group 12
| Case | Markdown path | C++ source | Intern command |
|---|---|---|---|
| 56 | `docs/test-cases/h265_vps_parser_unittest/h265vpsparsertest--testsamplevps2.md` | `test/h265_vps_parser_unittest.cc:99` | `Port test/h265_vps_parser_unittest.cc:99 using docs/test-cases/h265_vps_parser_unittest/h265vpsparsertest--testsamplevps2.md` |
| 57 | `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters.md` | `test/h265_vui_parameters_parser_unittest.cc:21` | `Port test/h265_vui_parameters_parser_unittest.cc:21 using docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters.md` |
| 58 | `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters2.md` | `test/h265_vui_parameters_parser_unittest.cc:61` | `Port test/h265_vui_parameters_parser_unittest.cc:61 using docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters2.md` |
| 59 | `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparametersnvenc.md` | `test/h265_vui_parameters_parser_unittest.cc:98` | `Port test/h265_vui_parameters_parser_unittest.cc:98 using docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparametersnvenc.md` |

## Run Log
| Group | Started (UTC) | Completed (UTC) | Notes |
|---|---|---|---|
| Group 01 | 2026-03-04T02:59:59Z | 2026-03-04T03:35:07Z | Ports landed in `crates/h265nal-test-ports/tests/`; AUD and GetSliceQpY cases now run, bitstream checksum/metadata cases still ignored with TODO. |
| Group 02 | 2026-03-04T02:59:59Z | 2026-03-04T03:35:07Z | Common helper and more_rbsp_data parity tests now run in `crates/h265nal-test-ports/tests/`. |
| Group 03 | 2026-03-04T03:35:07Z | 2026-03-04T03:35:07Z | Case files created for configuration-box, HRD, and NAL-unit coverage; currently ignored pending parser APIs. |
| Group 04 | 2026-03-04T03:35:07Z | 2026-03-04T03:37:05Z | All 5 case files created; currently ignored pending PPS/pred-weight parser APIs. |
| Group 05 | 2026-03-04T03:37:05Z | 2026-03-04T03:39:09Z | All 5 case files created; currently ignored pending pred-weight/profile-tier-level/RTP parser APIs. |
| Group 06 | 2026-03-04T03:39:09Z |  | Parallel intern fanout started (5 cases). |
| Between-group API closure (G03-G05) | 2026-03-04T03:39:09Z | 2026-03-04T04:10:00Z | Added NAL helper/parse core, profile_tier_level parse, and PPS parse C/Rust APIs; unlocked Case13, Case14, Case17, Case22, and Case23. |
| Group 07 |  |  |  |
| Group 08 |  |  |  |
| Group 09 |  |  |  |
| Group 10 |  |  |  |
| Group 11 |  |  |  |
| Group 12 |  |  |  |

## Counts Check
- Groups in this file: 12
- Cases in this file: 59
- Source index: `docs/test-cases/README.md` reports 59 test cases.
