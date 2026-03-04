//! References:
//! - Markdown: `docs/test-cases/h265_sps_3d_extension_parser_unittest/h265sps3dextensionparsertest--testsamplesps3dextension.md`
//! - C++: `test/h265_sps_3d_extension_parser_unittest.cc:21`
//! - Port note: Group 09 / Case 42

#[test]
#[ignore = "TODO: missing sps_3d_extension_parse API in h265nal-sys"]
fn test_sample_sps_3d_extension() {
    // TODO: Implement when sps_3d_extension_parse is exposed in h265nal-sys
    // sps_3d_extension
    // fuzzer::conv: data
    let buffer = [0xffu8, 0xff];
    // fuzzer::conv: begin
    // auto sps_3d_extension = H265Sps3dExtensionParser::ParseSps3dExtension(buffer, arraysize(buffer));
    // fuzzer::conv: end

    // EXPECT_TRUE(sps_3d_extension != nullptr);

    // EXPECT_THAT(sps_3d_extension->iv_di_mc_enabled_flag, ::testing::ElementsAreArray({1, 1}));
    // EXPECT_THAT(sps_3d_extension->iv_mv_scal_enabled_flag, ::testing::ElementsAreArray({1, 1}));
    // EXPECT_EQ(0, sps_3d_extension->log2_ivmc_sub_pb_size_minus3);
    // EXPECT_EQ(1, sps_3d_extension->iv_res_pred_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->depth_ref_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->vsp_mc_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->dbbp_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->tex_mc_enabled_flag);
    // EXPECT_EQ(0, sps_3d_extension->log2_texmc_sub_pb_size_minus3);
    // EXPECT_EQ(1, sps_3d_extension->intra_contour_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->intra_dc_only_wedge_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->cqt_cu_part_pred_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->inter_dc_only_enabled_flag);
    // EXPECT_EQ(1, sps_3d_extension->skip_intra_enabled_flag);
}
