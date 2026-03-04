//! References:
//! - Markdown: `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testcomplexstrefpicset.md`
//! - C++: `test/h265_sps_parser_unittest.cc:191`
//! - Port note: Group 09 / Case 44

#[test]
fn test_complex_st_ref_pic_set() {
    // Buffer from C++ test (46 bytes)
    let buffer = [
        0x01, 0x01, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x99, 0xa0, 0x03,
        0xc0, 0x80, 0x11, 0x07, 0xf9, 0x65, 0x26, 0x49, 0x1b, 0x61, 0xa5, 0x88, 0xaa, 0x93, 0x13,
        0x0c, 0xbe, 0xcf, 0xaf, 0x37, 0xe5, 0x9f, 0x5e, 0x14, 0x46, 0x27, 0x2e, 0xda, 0xc0, 0xff,
        0xff,
    ];

    let sps = h265nal_sys::sps_parse(&buffer).expect("ParseSps failed");

    assert_eq!(sps.sps_video_parameter_set_id, 0);
    assert_eq!(sps.sps_max_sub_layers_minus1, 0);
    assert_eq!(sps.sps_temporal_id_nesting_flag, 1);
    assert_eq!(sps.profile_tier_level.general.profile_idc, 1);
    assert_eq!(sps.profile_tier_level.general_level_idc, 153);
    assert_eq!(sps.chroma_format_idc, 1);
    assert_eq!(sps.pic_width_in_luma_samples, 1920);
    assert_eq!(sps.pic_height_in_luma_samples, 1088);
    assert_eq!(sps.sample_adaptive_offset_enabled_flag, 1);
    assert_eq!(sps.num_short_term_ref_pic_sets, 12);
    assert_eq!(sps.vui_parameters_present_flag, 0);
    assert_eq!(sps.sps_extension_present_flag, 1);
    assert_eq!(sps.sps_multilayer_extension_flag, 1);
    assert_eq!(sps.sps_3d_extension_flag, 1);
    assert_eq!(sps.st_ref_pic_set_size, 12);

    assert_eq!(sps.st_ref_pic_set[0].num_negative_pics, 4);
    assert_eq!(sps.st_ref_pic_set[0].num_positive_pics, 0);
    assert_eq!(sps.st_ref_pic_set[0].delta_poc_s0_minus1, [7, 1, 1, 3]);
    assert_eq!(sps.st_ref_pic_set[0].used_by_curr_pic_s0_flag, [1, 1, 1, 1]);

    assert_eq!(sps.st_ref_pic_set[1].inter_ref_pic_set_prediction_flag, 1);
    assert_eq!(sps.st_ref_pic_set[1].delta_rps_sign, 0);
    assert_eq!(sps.st_ref_pic_set[1].abs_delta_rps_minus1, 3);
    assert_eq!(sps.st_ref_pic_set[1].used_by_curr_pic_flag, [1, 1, 0, 0, 1]);
    assert_eq!(sps.st_ref_pic_set[1].use_delta_flag, [1, 1, 0, 0, 1]);

    assert_eq!(sps.st_ref_pic_set[11].inter_ref_pic_set_prediction_flag, 0);
    assert_eq!(sps.st_ref_pic_set[11].num_negative_pics, 0);
    assert_eq!(sps.st_ref_pic_set[11].num_positive_pics, 0);
    assert!(sps.st_ref_pic_set[11].delta_poc_s0_minus1.is_empty());
    assert!(sps.st_ref_pic_set[11].used_by_curr_pic_s0_flag.is_empty());
    assert!(sps.st_ref_pic_set[11].delta_poc_s1_minus1.is_empty());
    assert!(sps.st_ref_pic_set[11].used_by_curr_pic_s1_flag.is_empty());

    assert_eq!(sps.pic_size_in_ctbs_y, 510);
}
