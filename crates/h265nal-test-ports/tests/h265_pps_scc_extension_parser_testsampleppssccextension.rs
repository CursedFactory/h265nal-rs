//! References:
//! - Markdown: `docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testsampleppssccextension.md`
//! - C++: `test/h265_pps_scc_extension_parser_unittest.cc:21`
//! - Port note: Group 04 / Case 19

#[test]
fn test_sample_pps_scc_extension() {
    let buffer = [0x00u8];
    let pps_scc_extension =
        h265nal_sys::pps_scc_extension_parse(&buffer).expect("ParsePpsSccExtension failed");

    assert_eq!(pps_scc_extension.pps_curr_pic_ref_enabled_flag, 0);
    assert_eq!(
        pps_scc_extension.residual_adaptive_colour_transform_enabled_flag,
        0
    );
    assert_eq!(
        pps_scc_extension.pps_palette_predictor_initializer_present_flag,
        0
    );
    assert_eq!(pps_scc_extension.pps_palette_predictor_initializers_size, 0);
}
