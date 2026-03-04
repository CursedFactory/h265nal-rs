//! References:
//! - Markdown: `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testsamplesps.md`
//! - C++: `test/h265_sps_parser_unittest.cc:21`
//! - Port note: Group 09 / Case 45

#[test]
fn test_sample_sps() {
    // SPS for a 1280x736 camera capture.
    let buffer = [
        0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00, 0x03, 0x00,
        0x5d, 0xa0, 0x02, 0x80, 0x80, 0x2e, 0x1f, 0x13, 0x96, 0xbb, 0x93, 0x24, 0xbb, 0x95, 0x82,
        0x83, 0x03, 0x01, 0x76, 0x85, 0x09, 0x40,
    ];
    let sps = h265nal_sys::sps_parse(&buffer).expect("ParseSps failed");

    assert_eq!(sps.sps_video_parameter_set_id, 0);
    assert_eq!(sps.sps_max_sub_layers_minus1, 0);
    assert_eq!(sps.sps_temporal_id_nesting_flag, 1);
    assert_eq!(sps.profile_tier_level.general.profile_idc, 1);
    assert_eq!(sps.profile_tier_level.general_level_idc, 93);
    assert_eq!(sps.chroma_format_idc, 1);
    assert_eq!(sps.pic_width_in_luma_samples, 1280);
    assert_eq!(sps.pic_height_in_luma_samples, 736);
    assert_eq!(sps.conf_win_bottom_offset, 8);
    assert_eq!(sps.sample_adaptive_offset_enabled_flag, 1);
    assert_eq!(sps.num_short_term_ref_pic_sets, 1);
    assert_eq!(sps.sps_temporal_mvp_enabled_flag, 1);
    assert_eq!(sps.strong_intra_smoothing_enabled_flag, 1);
    assert_eq!(sps.vui_parameters_present_flag, 1);
    assert_eq!(sps.vui_video_format, 2);
    assert_eq!(sps.vui_colour_primaries, 5);
    assert_eq!(sps.sps_extension_present_flag, 0);
    assert_eq!(sps.pic_size_in_ctbs_y, 920);
}
