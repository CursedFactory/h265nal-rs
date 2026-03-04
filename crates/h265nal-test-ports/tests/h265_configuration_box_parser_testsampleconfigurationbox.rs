//! References:
//! - Markdown: `docs/test-cases/h265_configuration_box_parser_unittest/h265configurationboxparsertest--testsampleconfigurationbox.md`
//! - C++: `test/h265_configuration_box_parser_unittest.cc:21`
//! - Port note: Group 03 / Case 11

#[test]
fn test_sample_configuration_box() {
    // The buffer from the C++ test
    let buffer = [
        0x01, 0x01, 0x60, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0xf0, 0x00,
        0xfc, 0xfd, 0xf8, 0xf8, 0x00, 0x00, 0x0f,
        // offset: 22
        // unsigned int(8) numOfArrays;
        0x03,
        // offset: 23
        // for (j=0; j < numOfArrays; j++) {
        // bit(1) array_completeness;
        // unsigned int(1) reserved = 0;
        // unsigned int(6) NAL_unit_type; // 32 (VPS)
        0xa0, // unsigned int(16) numNalus;
        0x00, 0x01,
        // for (i=0; i< numNalus; i++) {
        // unsigned int(16) nalUnitLength;
        0x00, 0x18, // offset: 28
        // VPS-start
        0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0x80, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x78, 0x9d, 0xc0, 0x90,
        // VPS-end
        // offset: 52
        // for (j=0; j < numOfArrays; j++) {
        // bit(1) array_completeness;
        // unsigned int(1) reserved = 0;
        // unsigned int(6) NAL_unit_type; // 33 (SPS)
        0xa1, // unsigned int(16) numNalus;
        0x00, 0x01,
        // for (i=0; i< numNalus; i++) {
        // unsigned int(16) nalUnitLength;
        0x00, 0x27, // offset: 57
        // SPS-start
        0x42, 0x01, 0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0x80, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x03, 0x00, 0x78, 0xa0, 0x03, 0xc0, 0x80, 0x32, 0x16, 0x59, 0xde, 0x49, 0x1b, 0x6b, 0x80,
        0x40, 0x00, 0x00, 0xfa, 0x00, 0x00, 0x17, 0x70, 0x02,
        // SPS-end
        // for (j=0; j < numOfArrays; j++) {
        // bit(1) array_completeness;
        // unsigned int(1) reserved = 0;
        // unsigned int(6) NAL_unit_type; // 34 (PPS)
        0xa2, // unsigned int(16) numNalus;
        0x00, 0x01,
        // for (i=0; i< numNalus; i++) {
        // unsigned int(16) nalUnitLength;
        0x00, 0x06, // offset: 100
        // PPS-start
        0x44, 0x01, 0xc1, 0x73, 0xd1, 0x89, // PPS-end
    ];

    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    let configuration_box = h265nal_sys::configuration_box_parse(&buffer, &mut state)
        .expect("ParseConfigurationBox failed");

    assert_eq!(configuration_box.configuration_version, 1);
    assert_eq!(configuration_box.general_profile_space, 0);
    assert_eq!(configuration_box.general_tier_flag, 0);
    assert_eq!(configuration_box.general_profile_idc, 1);
    assert_eq!(
        configuration_box.general_profile_compatibility_flag,
        [
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]
    );
    assert_eq!(
        configuration_box.general_constraint_indicator_flags,
        0x800000000000
    );
    assert_eq!(configuration_box.general_level_idc, 120);
    assert_eq!(configuration_box.min_spatial_segmentation_idc, 0);
    assert_eq!(configuration_box.parallelism_type, 0);
    assert_eq!(configuration_box.chroma_format_idc, 1);
    assert_eq!(configuration_box.bit_depth_luma_minus8, 0);
    assert_eq!(configuration_box.bit_depth_chroma_minus8, 0);
    assert_eq!(configuration_box.avg_frame_rate, 0);
    assert_eq!(configuration_box.constant_frame_rate, 0);
    assert_eq!(configuration_box.num_temporal_layers, 1);
    assert_eq!(configuration_box.temporal_id_nested, 1);
    assert_eq!(configuration_box.length_size_minus_one, 3);
    assert_eq!(configuration_box.num_of_arrays, 3);
    assert_eq!(configuration_box.array_completeness_size, 3);
    assert_eq!(&configuration_box.array_completeness[..3], &[1, 1, 1]);
    assert_eq!(configuration_box.nal_unit_type_size, 3);
    assert_eq!(&configuration_box.nal_unit_type[..3], &[32, 33, 34]);
    assert_eq!(configuration_box.num_nalus_size, 3);
    assert_eq!(&configuration_box.num_nalus[..3], &[1, 1, 1]);
    assert_eq!(configuration_box.first_nal_unit_length_size, 3);
    assert_eq!(&configuration_box.first_nal_unit_length[..3], &[24, 39, 6]);
}
