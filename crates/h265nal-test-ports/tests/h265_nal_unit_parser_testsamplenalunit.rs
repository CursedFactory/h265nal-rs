//! References:
//! - Markdown: `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testsamplenalunit.md`
//! - C++: `test/h265_nal_unit_parser_unittest.cc:22`
//! - Port note: Group 03 / Case 15

#[test]
fn test_sample_nal_unit() {
    let buffer = [
        0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x5d, 0xac, 0x59, 0x00,
    ];

    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    let nal_unit =
        h265nal_sys::nal_unit_parse(&buffer, &mut state, true).expect("ParseNalUnit failed");

    assert_eq!(nal_unit.parsed_length, 20);
    assert_eq!(nal_unit.checksum, [0xbf, 0xa2, 0x45, 0x94]);
    assert_eq!(nal_unit.forbidden_zero_bit, 0);
    assert_eq!(nal_unit.nal_unit_type, 32);
    assert_eq!(nal_unit.nuh_layer_id, 0);
    assert_eq!(nal_unit.nuh_temporal_id_plus1, 1);

    let vps = h265nal_sys::vps_parse(&buffer[2..]).expect("ParseVps failed");
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
    assert_eq!(vps.profile_tier_level.general.progressive_source_flag, 1);
    assert_eq!(vps.profile_tier_level.general.interlaced_source_flag, 0);
    assert_eq!(vps.profile_tier_level.general.non_packed_constraint_flag, 1);
    assert_eq!(vps.profile_tier_level.general.frame_only_constraint_flag, 1);
    assert_eq!(vps.profile_tier_level.general_level_idc, 93);
}
