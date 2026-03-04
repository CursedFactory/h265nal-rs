//! References:
//! - Markdown: `docs/test-cases/h265_sps_range_extension_parser_unittest/h265spsrangeextensionparsertest--testsamplespsrangeextension.md`
//! - C++: `test/h265_sps_range_extension_parser_unittest.cc:21`
//! - Port note: Group 10 / Case 48

#[test]
fn test_sample_sps_range_extension() {
    let buffer = [0xaa, 0xaa];
    let sps_range_extension =
        h265nal_sys::sps_range_extension_parse(&buffer).expect("ParseSpsRangeExtension failed");

    assert_eq!(sps_range_extension.transform_skip_rotation_enabled_flag, 1);
    assert_eq!(sps_range_extension.transform_skip_context_enabled_flag, 0);
    assert_eq!(sps_range_extension.implicit_rdpcm_enabled_flag, 1);
    assert_eq!(sps_range_extension.explicit_rdpcm_enabled_flag, 0);
    assert_eq!(sps_range_extension.extended_precision_processing_flag, 1);
    assert_eq!(sps_range_extension.intra_smoothing_disabled_flag, 0);
    assert_eq!(sps_range_extension.high_precision_offsets_enabled_flag, 1);
    assert_eq!(
        sps_range_extension.persistent_rice_adaptation_enabled_flag,
        0
    );
    assert_eq!(sps_range_extension.cabac_bypass_alignment_enabled_flag, 1);
}
