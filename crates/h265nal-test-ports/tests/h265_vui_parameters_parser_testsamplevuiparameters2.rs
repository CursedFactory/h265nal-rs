//! References:
//! - Markdown: `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparameters2.md`
//! - C++: `test/h265_vui_parameters_parser_unittest.cc:61`
//! - Port note: Group 12 / Case 58

#[test]
fn test_sample_vui_parameters2() {
    let buffer = [
        0xff, 0x80, 0x40, 0x00, 0x3a, 0xb4, 0x04, 0x00, 0x00, 0x0f, 0xa4, 0x00, 0x01, 0xd4, 0xc0,
        0x20,
    ];
    let sps_max_sub_layers_minus1 = 0;
    let vui_parameters = h265nal_sys::vui_parameters_parse(&buffer, sps_max_sub_layers_minus1)
        .expect("ParseVuiParameters failed");

    assert_eq!(vui_parameters.aspect_ratio_info_present_flag, 1);
    assert_eq!(vui_parameters.aspect_ratio_idc, 255);
    assert_eq!(vui_parameters.sar_width, 128);
    assert_eq!(vui_parameters.sar_height, 117);
    assert_eq!(vui_parameters.overscan_info_present_flag, 0);
    assert_eq!(vui_parameters.video_signal_type_present_flag, 1);
    assert_eq!(vui_parameters.video_format, 5);
    assert_eq!(vui_parameters.video_full_range_flag, 0);
    assert_eq!(vui_parameters.colour_description_present_flag, 0);
    assert_eq!(vui_parameters.chroma_loc_info_present_flag, 0);
    assert_eq!(vui_parameters.neutral_chroma_indication_flag, 0);
    assert_eq!(vui_parameters.field_seq_flag, 0);
    assert_eq!(vui_parameters.frame_field_info_present_flag, 0);
    assert_eq!(vui_parameters.default_display_window_flag, 0);
    assert_eq!(vui_parameters.vui_timing_info_present_flag, 1);
    assert_eq!(vui_parameters.vui_num_units_in_tick, 1001);
    assert_eq!(vui_parameters.vui_time_scale, 30000);
    assert_eq!(vui_parameters.vui_poc_proportional_to_timing_flag, 0);
    assert_eq!(vui_parameters.vui_hrd_parameters_present_flag, 0);
    assert_eq!(vui_parameters.bitstream_restriction_flag, 0);
}
