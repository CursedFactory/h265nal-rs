//! References:
//! - Markdown: `docs/test-cases/h265_vps_parser_unittest/h265vpsparsertest--testsamplevps2.md`
//! - C++: `test/h265_vps_parser_unittest.cc:99`
//! - Port note: Group 12 / Case 56

#[test]
fn test_sample_vps2() {
    let buffer = [
        0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00,
        0x00, 0x03, 0x00, 0x5d, 0xac, 0x56, 0x07, 0xe4, 0x00,
    ];

    let vps = h265nal_sys::vps_parse(&buffer).expect("ParseVps failed");

    assert_eq!(vps.vps_video_parameter_set_id, 0);
    assert_eq!(vps.vps_base_layer_internal_flag, 1);
    assert_eq!(vps.vps_base_layer_available_flag, 1);
    assert_eq!(vps.vps_max_layers_minus1, 0);
    assert_eq!(vps.vps_max_sub_layers_minus1, 0);
    assert_eq!(vps.vps_temporal_id_nesting_flag, 1);
    assert_eq!(vps.vps_reserved_0xffff_16bits, 0xffff);
    assert_eq!(vps.profile_tier_level.general.profile_space, 0);
    assert_eq!(vps.profile_tier_level.general.tier_flag, 0);
    assert_eq!(vps.profile_tier_level.general.profile_idc, 1);
    assert_eq!(
        vps.profile_tier_level.general.profile_compatibility_flag,
        [
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );
    assert_eq!(vps.profile_tier_level.general.progressive_source_flag, 1);
    assert_eq!(vps.profile_tier_level.general.interlaced_source_flag, 0);
    assert_eq!(vps.profile_tier_level.general.non_packed_constraint_flag, 1);
    assert_eq!(vps.profile_tier_level.general.frame_only_constraint_flag, 1);
    assert_eq!(vps.profile_tier_level.general.max_12bit_constraint_flag, 0);
    assert_eq!(vps.profile_tier_level.general.max_10bit_constraint_flag, 0);
    assert_eq!(vps.profile_tier_level.general.max_8bit_constraint_flag, 0);
    assert_eq!(
        vps.profile_tier_level.general.max_422chroma_constraint_flag,
        0
    );
    assert_eq!(
        vps.profile_tier_level.general.max_420chroma_constraint_flag,
        0
    );
    assert_eq!(
        vps.profile_tier_level
            .general
            .max_monochrome_constraint_flag,
        0
    );
    assert_eq!(vps.profile_tier_level.general.intra_constraint_flag, 0);
    assert_eq!(
        vps.profile_tier_level
            .general
            .one_picture_only_constraint_flag,
        0
    );
    assert_eq!(
        vps.profile_tier_level
            .general
            .lower_bit_rate_constraint_flag,
        0
    );
    assert_eq!(vps.profile_tier_level.general.max_14bit_constraint_flag, 0);
    assert_eq!(vps.profile_tier_level.general.reserved_zero_33bits, 0);
    assert_eq!(vps.profile_tier_level.general.reserved_zero_34bits, 0);
    assert_eq!(vps.profile_tier_level.general.reserved_zero_43bits, 0);
    assert_eq!(vps.profile_tier_level.general.inbld_flag, 0);
    assert_eq!(vps.profile_tier_level.general.reserved_zero_bit, 0);
    assert_eq!(vps.profile_tier_level.general_level_idc, 93);
    assert_eq!(
        vps.profile_tier_level.sub_layer_profile_present_flag_size,
        0
    );
    assert_eq!(vps.profile_tier_level.sub_layer_level_present_flag_size, 0);
    assert_eq!(vps.profile_tier_level.reserved_zero_2bits_size, 0);
    assert_eq!(vps.profile_tier_level.sub_layer_size, 0);
    assert_eq!(vps.vps_sub_layer_ordering_info_present_flag, 1);
    assert_eq!(vps.vps_max_dec_pic_buffering_minus1_size, 1);
    assert_eq!(vps.vps_max_dec_pic_buffering_minus1[0], 1);
    assert_eq!(vps.vps_max_num_reorder_pics_size, 1);
    assert_eq!(vps.vps_max_num_reorder_pics[0], 0);
    assert_eq!(vps.vps_max_latency_increase_plus1_size, 1);
    assert_eq!(vps.vps_max_latency_increase_plus1[0], 0);
    assert_eq!(vps.vps_max_layer_id, 5);
    assert_eq!(vps.vps_num_layer_sets_minus1, 2);
    assert_eq!(vps.layer_id_included_flag_size, 2);
    assert_eq!(vps.layer_id_included_flag_row_width, 6);
    assert_eq!(vps.layer_id_included_flag_row0_mask, 0);
    assert_eq!(vps.layer_id_included_flag_row1_mask, 0b11_1111);
    assert_eq!(vps.vps_timing_info_present_flag, 0);
    assert_eq!(vps.vps_num_units_in_tick, 0);
    assert_eq!(vps.vps_time_scale, 0);
    assert_eq!(vps.vps_poc_proportional_to_timing_flag, 0);
    assert_eq!(vps.vps_num_ticks_poc_diff_one_minus1, 0);
    assert_eq!(vps.vps_num_hrd_parameters, 0);
    assert_eq!(vps.hrd_layer_set_idx_size, 0);
    assert_eq!(vps.cprms_present_flag_size, 0);
    assert_eq!(vps.vps_extension_flag, 0);
    assert_eq!(vps.vps_extension_data_flag, 0);
}
