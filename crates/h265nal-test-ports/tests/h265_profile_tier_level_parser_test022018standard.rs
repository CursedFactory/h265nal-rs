//! References:
//! - Markdown: `docs/test-cases/h265_profile_tier_level_parser_unittest/h265profiletierlevelparsertest--test022018standard.md`
//! - C++: `test/h265_profile_tier_level_parser_unittest.cc:69`
//! - Port note: Group 05 / Case 22

#[test]
fn test_022018_standard() {
    // fuzzer::conv: data
    let buffer = [
        0x01, 0x60, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0xba, 0x00, 0x00, 0x2c,
        0x09,
    ];
    let ptls = h265nal_sys::profile_tier_level_parse(&buffer, true, 0)
        .expect("ParseProfileTierLevel failed");

    assert_eq!(ptls.general.profile_space, 0);
    assert_eq!(ptls.general.tier_flag, 0);
    assert_eq!(ptls.general.profile_idc, 1);
    assert_eq!(
        ptls.general.profile_compatibility_flag,
        [
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0
        ]
    );
    assert_eq!(ptls.general.progressive_source_flag, 1);
    assert_eq!(ptls.general.interlaced_source_flag, 0);
    assert_eq!(ptls.general.non_packed_constraint_flag, 0);
    assert_eq!(ptls.general.frame_only_constraint_flag, 0);
    assert_eq!(ptls.general.max_12bit_constraint_flag, 0);
    assert_eq!(ptls.general.max_10bit_constraint_flag, 0);
    assert_eq!(ptls.general.max_8bit_constraint_flag, 0);
    assert_eq!(ptls.general.max_422chroma_constraint_flag, 0);
    assert_eq!(ptls.general.max_420chroma_constraint_flag, 0);
    assert_eq!(ptls.general.max_monochrome_constraint_flag, 0);
    assert_eq!(ptls.general.intra_constraint_flag, 0);
    assert_eq!(ptls.general.one_picture_only_constraint_flag, 0);
    assert_eq!(ptls.general.lower_bit_rate_constraint_flag, 0);
    assert_eq!(ptls.general.max_14bit_constraint_flag, 0);
    assert_eq!(ptls.general.reserved_zero_33bits, 0);
    assert_eq!(ptls.general.reserved_zero_34bits, 0);
    assert_eq!(ptls.general.reserved_zero_43bits, 0);
    assert_eq!(ptls.general.inbld_flag, 0);
    assert_eq!(ptls.general.reserved_zero_bit, 0);
    assert_eq!(ptls.general_level_idc, 186);

    assert_eq!(ptls.sub_layer_profile_present_flag_size, 0);
    assert_eq!(ptls.sub_layer_level_present_flag_size, 0);
    assert_eq!(ptls.reserved_zero_2bits_size, 0);
    assert_eq!(ptls.sub_layer_size, 0);
}
