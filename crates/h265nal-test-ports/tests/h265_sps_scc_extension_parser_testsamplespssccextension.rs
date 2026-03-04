//! References:
//! - Markdown: `docs/test-cases/h265_sps_scc_extension_parser_unittest/h265spssccextensionparsertest--testsamplespssccextension.md`
//! - C++: `test/h265_sps_scc_extension_parser_unittest.cc:21`
//! - Port note: Group 10 / Case 49

#[test]
fn test_sample_sps_scc_extension() {
    let buffer = [0x80];
    let sps_scc_extension = h265nal_sys::sps_scc_extension_parse(
        &buffer, 1, // sps->chroma_format_idc
        0, // sps->bit_depth_luma_minus8
        0, // sps->bit_depth_chroma_minus8
    )
    .expect("ParseSpsSccExtension failed");

    assert_eq!(sps_scc_extension.sps_curr_pic_ref_enabled_flag, 1);
    assert_eq!(sps_scc_extension.palette_mode_enabled_flag, 0);
    assert_eq!(sps_scc_extension.palette_max_size, 0);
    assert_eq!(sps_scc_extension.delta_palette_max_predictor_size, 0);
    assert_eq!(
        sps_scc_extension.sps_palette_predictor_initializers_present_flag,
        0
    );
    assert_eq!(
        sps_scc_extension.sps_num_palette_predictor_initializers_minus1,
        0
    );
    assert_eq!(sps_scc_extension.sps_palette_predictor_initializers_size, 0);
    assert_eq!(sps_scc_extension.motion_vector_resolution_control_idc, 0);
    assert_eq!(sps_scc_extension.intra_boundary_filtering_disabled_flag, 0);
}
