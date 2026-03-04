//! References:
//! - Markdown: `docs/test-cases/h265_configuration_box_parser_unittest/h265configurationboxparsertest--testsampleconfigurationbox.md`
//! - C++: `test/h265_configuration_box_parser_unittest.cc:21`
//! - Port note: Group 03 / Case 11

// TODO: missing h265nal_sys::configuration_box_parse or similar API

#[ignore = "TODO: missing configuration box parser API in h265nal-sys"]
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

    // TODO: Once API is available, uncomment and implement:
    // let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    // let parsing_options = h265nal_sys::ParsingOptions::default();
    // let configuration_box = h265nal_sys::configuration_box_parse(&buffer, &mut state, parsing_options).expect("ParseConfigurationBox failed");
    //
    // assert!(configuration_box.is_some());
    // let config = configuration_box.unwrap();
    // assert_eq!(config.configuration_version, 1);
    // // ... many more assertions as in C++ test

    // Placeholder to make it compile
    let _buffer = buffer;
}
