//! References:
//! - Markdown: `docs/test-cases/h265_sub_layer_hrd_parameters_parser_unittest/h265sublayerhrdparametersparsertest--testsamplesublayerhrdparameters.md`
//! - C++: `test/h265_sub_layer_hrd_parameters_parser_unittest.cc:21`
//! - Port note: Group 11 / Case 51

#[test]
fn test_sample_sub_layer_hrd_parameters() {
    let buffer = [0x00, 0x01, 0x6e, 0x36, 0x00, 0x01, 0xe8, 0x48, 0x10];

    let sub_layer_hrd_parameters =
        h265nal_sys::sub_layer_hrd_parameters_parse(&buffer, 0, 1, false)
            .expect("ParseSubLayerHrdParameters failed");

    assert_eq!(sub_layer_hrd_parameters.bit_rate_value_minus1_size, 1);
    assert_eq!(sub_layer_hrd_parameters.bit_rate_value_minus1[0], 46874);
    assert_eq!(sub_layer_hrd_parameters.cpb_size_value_minus1_size, 1);
    assert_eq!(sub_layer_hrd_parameters.cpb_size_value_minus1[0], 124999);
    assert_eq!(sub_layer_hrd_parameters.cpb_size_du_value_minus1_size, 1);
    assert_eq!(sub_layer_hrd_parameters.cpb_size_du_value_minus1[0], 0);
    assert_eq!(sub_layer_hrd_parameters.bit_rate_du_value_minus1_size, 1);
    assert_eq!(sub_layer_hrd_parameters.bit_rate_du_value_minus1[0], 0);
    assert_eq!(sub_layer_hrd_parameters.cbr_flag_size, 1);
    assert_eq!(sub_layer_hrd_parameters.cbr_flag[0], 0);
}
