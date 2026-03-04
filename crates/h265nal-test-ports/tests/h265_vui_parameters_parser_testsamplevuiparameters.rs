//! References:
//! - Markdown: `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters.md`
//! - C++: `test/h265_vui_parameters_parser_unittest.cc:21`
//! - Port note: Group 12 / Case 57

#[test]
fn test_sample_vui_parameters() {
    let buffer = [0x2b, 0x05, 0x06, 0x06, 0x02, 0xed, 0x0a, 0x12];
    let sps_max_sub_layers_minus1 = 0;
    let vui_parameters = h265nal_sys::vui_parameters_parse(&buffer, sps_max_sub_layers_minus1)
        .expect("ParseVuiParameters failed");

    assert_eq!(vui_parameters.aspect_ratio_info_present_flag, 0);
    assert_eq!(vui_parameters.overscan_info_present_flag, 0);
    assert_eq!(vui_parameters.video_signal_type_present_flag, 1);
    assert_eq!(vui_parameters.video_format, 2);
    assert_eq!(vui_parameters.video_full_range_flag, 1);
    assert_eq!(vui_parameters.colour_description_present_flag, 1);
    assert_eq!(vui_parameters.colour_primaries, 5);
    assert_eq!(vui_parameters.transfer_characteristics, 6);
    assert_eq!(vui_parameters.matrix_coeffs, 6);
    assert_eq!(vui_parameters.chroma_loc_info_present_flag, 0);
    assert_eq!(vui_parameters.neutral_chroma_indication_flag, 0);
    assert_eq!(vui_parameters.field_seq_flag, 0);
    assert_eq!(vui_parameters.frame_field_info_present_flag, 0);
    assert_eq!(vui_parameters.default_display_window_flag, 0);
    assert_eq!(vui_parameters.vui_timing_info_present_flag, 0);
    assert_eq!(vui_parameters.bitstream_restriction_flag, 1);
    assert_eq!(vui_parameters.tiles_fixed_structure_flag, 0);
    assert_eq!(vui_parameters.motion_vectors_over_pic_boundaries_flag, 1);
    assert_eq!(vui_parameters.restricted_ref_pic_lists_flag, 1);
    assert_eq!(vui_parameters.min_spatial_segmentation_idc, 0);
    assert_eq!(vui_parameters.max_bytes_per_pic_denom, 2);
    assert_eq!(vui_parameters.max_bits_per_min_cu_denom, 1);
    assert_eq!(vui_parameters.log2_max_mv_length_horizontal, 9);
    assert_eq!(vui_parameters.log2_max_mv_length_vertical, 8);
}
