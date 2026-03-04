//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataunregisteredsei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:50`
//! - Port note: Group 08 / Case 37

#[test]
#[ignore = "TODO: missing SEI parsing API in h265nal-sys"]
fn test_user_data_unregistered_sei() {
    // Test data for user data unregistered SEI
    let _buffer = [
        0x05, 0x38, 0x2c, 0xa2, 0xde, 0x09, 0xb5, 0x17, 0x47, 0xdb, 0xbb, 0x55, 0xa4, 0xfe, 0x7f,
        0xc2, 0xfc, 0x4e, 0x78, 0x32, 0x36, 0x35, 0x20, 0x28, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x20,
        0x33, 0x31, 0x29, 0x20, 0x2d, 0x20, 0x31, 0x2e, 0x33, 0x2b, 0x32, 0x30, 0x2d, 0x36, 0x65,
        0x36, 0x37, 0x35, 0x36, 0x66, 0x39, 0x34, 0x62, 0x32, 0x37, 0x3a, 0x5b, 0x57, 0x69,
    ];

    // TODO: Implement SEI parsing API in h265nal-sys
    // auto sei_message = H265SeiMessageParser::ParseSei(buffer, arraysize(buffer));
    // EXPECT_TRUE(sei_message != nullptr);
    // EXPECT_EQ(sei_message->payload_type, h265nal::SeiType::user_data_unregistered);
    // EXPECT_EQ(sei_message->payload_size, 56);
    // auto user_data_unregistered_sei = dynamic_cast<H265SeiUserDataUnregisteredParser::H265SeiUserDataUnregisteredState*>(sei_message->payload_state.get());
    // EXPECT_TRUE(user_data_unregistered_sei != nullptr);
    // EXPECT_EQ(user_data_unregistered_sei->uuid_iso_iec_11578_1, 0x2ca2de09b51747db);
    // EXPECT_EQ(user_data_unregistered_sei->uuid_iso_iec_11578_2, 0xbb55a4fe7fc2fc4e);

    todo!("SEI parsing API not yet implemented in h265nal-sys");
}
