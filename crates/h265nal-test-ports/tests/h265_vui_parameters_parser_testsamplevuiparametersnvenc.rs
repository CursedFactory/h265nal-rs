//! References:
//! - Markdown: `docs/test-cases/h265_vui_parameters_parser_unittest/h265vuiparametersparsertest--testsamplevuiparametersnvenc.md`
//! - C++: `test/h265_vui_parameters_parser_unittest.cc:98`
//! - Port note: Group 12 / Case 59

#[test]
fn test_sample_vui_parameters_nvenc() {
    // VUI
    // fuzzer::conv: data
    let buffer = [
        0x80, 0x80, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x1e, 0x30, 0x02, 0xef, 0x28, 0x80,
        0x00, 0xb7, 0x1b, 0x00, 0x00, 0xf4, 0x24, 0x00,
    ];

    let sps_max_sub_layers_minus1 = 0;
    let vui_parameters = h265nal_sys::vui_parameters_parse(&buffer, sps_max_sub_layers_minus1)
        .expect("ParseVuiParameters failed");

    assert_eq!(vui_parameters.aspect_ratio_info_present_flag, 1);
    assert_eq!(vui_parameters.aspect_ratio_idc, 1);
    assert_eq!(vui_parameters.sar_width, 0);
    assert_eq!(vui_parameters.sar_height, 0);

    assert_eq!(vui_parameters.overscan_info_present_flag, 0);
    assert_eq!(vui_parameters.video_signal_type_present_flag, 0);
    assert_eq!(vui_parameters.video_format, 0);
    assert_eq!(vui_parameters.video_full_range_flag, 0);
    assert_eq!(vui_parameters.colour_description_present_flag, 0);
    assert_eq!(vui_parameters.chroma_loc_info_present_flag, 0);
    assert_eq!(vui_parameters.neutral_chroma_indication_flag, 0);
    assert_eq!(vui_parameters.field_seq_flag, 0);
    assert_eq!(vui_parameters.frame_field_info_present_flag, 0);
    assert_eq!(vui_parameters.default_display_window_flag, 0);
    assert_eq!(vui_parameters.vui_timing_info_present_flag, 1);
    assert_eq!(vui_parameters.vui_num_units_in_tick, 1);
    assert_eq!(vui_parameters.vui_time_scale, 60);
    assert_eq!(vui_parameters.vui_poc_proportional_to_timing_flag, 0);
    assert_eq!(vui_parameters.vui_hrd_parameters_present_flag, 1);
    assert_eq!(vui_parameters.bitstream_restriction_flag, 0);
    assert_eq!(vui_parameters.has_hrd_parameters, 1);
    assert_eq!(vui_parameters.hrd_nal_hrd_parameters_present_flag, 1);
    assert_eq!(vui_parameters.hrd_vcl_hrd_parameters_present_flag, 0);
    assert_eq!(vui_parameters.hrd_sub_pic_hrd_params_present_flag, 0);
    assert_eq!(vui_parameters.hrd_tick_divisor_minus2, 0);
    assert_eq!(
        vui_parameters.hrd_du_cpb_removal_delay_increment_length_minus1,
        0
    );
    assert_eq!(
        vui_parameters.hrd_sub_pic_cpb_params_in_pic_timing_sei_flag,
        0
    );
    assert_eq!(vui_parameters.hrd_dpb_output_delay_du_length_minus1, 0);
    assert_eq!(vui_parameters.hrd_bit_rate_scale, 0);
    assert_eq!(vui_parameters.hrd_cpb_size_scale, 0);
    assert_eq!(vui_parameters.hrd_cpb_size_du_scale, 0);
    assert_eq!(
        vui_parameters.hrd_initial_cpb_removal_delay_length_minus1,
        23
    );
    assert_eq!(vui_parameters.hrd_au_cpb_removal_delay_length_minus1, 15);
    assert_eq!(vui_parameters.hrd_dpb_output_delay_length_minus1, 5);
    assert_eq!(vui_parameters.hrd_fixed_pic_rate_general_flag_size, 1);
    assert_eq!(vui_parameters.hrd_fixed_pic_rate_general_flag[0], 0);
    assert_eq!(vui_parameters.hrd_fixed_pic_rate_within_cvs_flag_size, 1);
    assert_eq!(vui_parameters.hrd_fixed_pic_rate_within_cvs_flag[0], 0);
    assert_eq!(vui_parameters.hrd_elemental_duration_in_tc_minus1_size, 1);
    assert_eq!(vui_parameters.hrd_elemental_duration_in_tc_minus1[0], 0);
    assert_eq!(vui_parameters.hrd_low_delay_hrd_flag_size, 1);
    assert_eq!(vui_parameters.hrd_low_delay_hrd_flag[0], 0);
    assert_eq!(vui_parameters.hrd_cpb_cnt_minus1_size, 1);
    assert_eq!(vui_parameters.hrd_cpb_cnt_minus1[0], 0);

    assert_eq!(vui_parameters.hrd_sub_layer_hrd_parameters_vector_size, 1);
    assert_eq!(vui_parameters.hrd_sub_layer_hrd_parameters_0_present, 1);
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .bit_rate_value_minus1_size,
        1
    );
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .bit_rate_value_minus1[0],
        46874
    );
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .cpb_size_value_minus1_size,
        1
    );
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .cpb_size_value_minus1[0],
        124999
    );
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .cpb_size_du_value_minus1_size,
        1
    );
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .cpb_size_du_value_minus1[0],
        0
    );
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .bit_rate_du_value_minus1_size,
        1
    );
    assert_eq!(
        vui_parameters
            .hrd_sub_layer_hrd_parameters_0
            .bit_rate_du_value_minus1[0],
        0
    );
    assert_eq!(
        vui_parameters.hrd_sub_layer_hrd_parameters_0.cbr_flag_size,
        1
    );
    assert_eq!(vui_parameters.hrd_sub_layer_hrd_parameters_0.cbr_flag[0], 0);
}
