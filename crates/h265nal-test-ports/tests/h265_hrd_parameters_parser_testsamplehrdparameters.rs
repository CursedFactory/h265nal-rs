//! References:
//! - Markdown: `docs/test-cases/h265_hrd_parameters_parser_unittest/h265hrdparametersparsertest--testsamplehrdparameters.md`
//! - C++: `test/h265_hrd_parameters_parser_unittest.cc:21`
//! - Port note: Group 03 / Case 12

#[test]
fn test_sample_hrd_parameters() {
    let buffer = [
        0x80, 0x17, 0x79, 0x44, 0x00, 0x05, 0xb8, 0xd8, 0x00, 0x07, 0xa1, 0x20, 0x40, 0x00,
    ];

    let hrd_parameters =
        h265nal_sys::hrd_parameters_parse(&buffer, 1, 0).expect("ParseHrdParameters failed");

    assert_eq!(hrd_parameters.nal_hrd_parameters_present_flag, 1);
    assert_eq!(hrd_parameters.vcl_hrd_parameters_present_flag, 0);
    assert_eq!(hrd_parameters.sub_pic_hrd_params_present_flag, 0);
    assert_eq!(hrd_parameters.tick_divisor_minus2, 0);
    assert_eq!(
        hrd_parameters.du_cpb_removal_delay_increment_length_minus1,
        0
    );
    assert_eq!(hrd_parameters.sub_pic_cpb_params_in_pic_timing_sei_flag, 0);
    assert_eq!(hrd_parameters.dpb_output_delay_du_length_minus1, 0);
    assert_eq!(hrd_parameters.bit_rate_scale, 0);
    assert_eq!(hrd_parameters.cpb_size_scale, 0);
    assert_eq!(hrd_parameters.cpb_size_du_scale, 0);
    assert_eq!(hrd_parameters.initial_cpb_removal_delay_length_minus1, 23);
    assert_eq!(hrd_parameters.au_cpb_removal_delay_length_minus1, 15);
    assert_eq!(hrd_parameters.dpb_output_delay_length_minus1, 5);
    assert_eq!(hrd_parameters.fixed_pic_rate_general_flag_size, 1);
    assert_eq!(hrd_parameters.fixed_pic_rate_general_flag[0], 0);
    assert_eq!(hrd_parameters.fixed_pic_rate_within_cvs_flag_size, 1);
    assert_eq!(hrd_parameters.fixed_pic_rate_within_cvs_flag[0], 0);
    assert_eq!(hrd_parameters.elemental_duration_in_tc_minus1_size, 1);
    assert_eq!(hrd_parameters.elemental_duration_in_tc_minus1[0], 0);
    assert_eq!(hrd_parameters.low_delay_hrd_flag_size, 1);
    assert_eq!(hrd_parameters.low_delay_hrd_flag[0], 0);
    assert_eq!(hrd_parameters.cpb_cnt_minus1_size, 1);
    assert_eq!(hrd_parameters.cpb_cnt_minus1[0], 0);

    assert_eq!(hrd_parameters.sub_layer_hrd_parameters_vector_size, 1);
    assert_eq!(hrd_parameters.sub_layer_hrd_parameters_0_present, 1);
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .bit_rate_value_minus1_size,
        1
    );
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .bit_rate_value_minus1[0],
        46874
    );
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .cpb_size_value_minus1_size,
        1
    );
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .cpb_size_value_minus1[0],
        124999
    );
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .cpb_size_du_value_minus1_size,
        1
    );
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .cpb_size_du_value_minus1[0],
        0
    );
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .bit_rate_du_value_minus1_size,
        1
    );
    assert_eq!(
        hrd_parameters
            .sub_layer_hrd_parameters_0
            .bit_rate_du_value_minus1[0],
        0
    );
    assert_eq!(hrd_parameters.sub_layer_hrd_parameters_0.cbr_flag_size, 1);
    assert_eq!(hrd_parameters.sub_layer_hrd_parameters_0.cbr_flag[0], 0);
}
