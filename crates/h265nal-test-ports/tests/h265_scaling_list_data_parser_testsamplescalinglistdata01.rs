//! References:
//! - Markdown: `docs/test-cases/h265_scaling_list_data_parser_unittest/h265scalinglistdataparsertest--testsamplescalinglistdata01.md`
//! - C++: `test/h265_scaling_list_data_parser_unittest.cc:21`
//! - Port note: Group 07 / Case 32

#[test]
fn test_sample_scaling_list_data01() {
    let buffer = [0xffu8; 256];
    let scaling_list_data =
        h265nal_sys::scaling_list_data_parse(&buffer).expect("ParseScalingListData failed");

    assert_eq!(scaling_list_data.size_id_count, 4);
    assert_eq!(scaling_list_data.matrix_id_count, 6);

    for matrix_id in 0..6 {
        let idx = matrix_id;
        assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[idx], 1);
    }
    for matrix_id in 0..6 {
        let idx = 6 + matrix_id;
        assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[idx], 1);
    }
    for matrix_id in 0..6 {
        let idx = 12 + matrix_id;
        assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[idx], 1);
    }
    assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[18], 1);
    assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[19], 0);
    assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[20], 0);
    assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[21], 1);
    assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[22], 0);
    assert_eq!(scaling_list_data.scaling_list_pred_mode_flag[23], 0);

    for idx in 0..24 {
        assert_eq!(scaling_list_data.scaling_list_pred_matrix_id_delta[idx], 0);
        assert_eq!(scaling_list_data.scaling_list_dc_coef_minus8[idx], 0);
    }

    for idx in 0..18 {
        let expected_size = if idx < 6 { 16 } else { 64 };
        assert_eq!(scaling_list_data.scaling_list_size[idx], expected_size);
        assert_eq!(scaling_list_data.scaling_list_first_coef[idx], 8);
        assert_eq!(scaling_list_data.scaling_list_all_8[idx], 1);
    }

    assert_eq!(scaling_list_data.scaling_list_size[18], 64);
    assert_eq!(scaling_list_data.scaling_list_first_coef[18], 8);
    assert_eq!(scaling_list_data.scaling_list_all_8[18], 1);
    assert_eq!(scaling_list_data.scaling_list_size[19], 0);
    assert_eq!(scaling_list_data.scaling_list_size[20], 0);
    assert_eq!(scaling_list_data.scaling_list_size[21], 64);
    assert_eq!(scaling_list_data.scaling_list_first_coef[21], 8);
    assert_eq!(scaling_list_data.scaling_list_all_8[21], 1);
    assert_eq!(scaling_list_data.scaling_list_size[22], 0);
    assert_eq!(scaling_list_data.scaling_list_size[23], 0);
}
