//! References:
//! - Markdown: `docs/test-cases/h265_sps_3d_extension_parser_unittest/h265sps3dextensionparsertest--testsamplesps3dextension.md`
//! - C++: `test/h265_sps_3d_extension_parser_unittest.cc:21`
//! - Port note: Group 09 / Case 42

#[test]
fn test_sample_sps_3d_extension() {
    let buffer = [0xffu8, 0xff];
    let sps_3d_extension =
        h265nal_sys::sps_3d_extension_parse(&buffer).expect("ParseSps3dExtension failed");

    assert_eq!(sps_3d_extension.iv_di_mc_enabled_flag_size, 2);
    assert_eq!(sps_3d_extension.iv_di_mc_enabled_flag[0], 1);
    assert_eq!(sps_3d_extension.iv_di_mc_enabled_flag[1], 1);
    assert_eq!(sps_3d_extension.iv_mv_scal_enabled_flag_size, 2);
    assert_eq!(sps_3d_extension.iv_mv_scal_enabled_flag[0], 1);
    assert_eq!(sps_3d_extension.iv_mv_scal_enabled_flag[1], 1);
    assert_eq!(sps_3d_extension.log2_ivmc_sub_pb_size_minus3, 0);
    assert_eq!(sps_3d_extension.iv_res_pred_enabled_flag, 1);
    assert_eq!(sps_3d_extension.depth_ref_enabled_flag, 1);
    assert_eq!(sps_3d_extension.vsp_mc_enabled_flag, 1);
    assert_eq!(sps_3d_extension.dbbp_enabled_flag, 1);
    assert_eq!(sps_3d_extension.tex_mc_enabled_flag, 1);
    assert_eq!(sps_3d_extension.log2_texmc_sub_pb_size_minus3, 0);
    assert_eq!(sps_3d_extension.intra_contour_enabled_flag, 1);
    assert_eq!(sps_3d_extension.intra_dc_only_wedge_enabled_flag, 1);
    assert_eq!(sps_3d_extension.cqt_cu_part_pred_enabled_flag, 1);
    assert_eq!(sps_3d_extension.inter_dc_only_enabled_flag, 1);
    assert_eq!(sps_3d_extension.skip_intra_enabled_flag, 1);
}
