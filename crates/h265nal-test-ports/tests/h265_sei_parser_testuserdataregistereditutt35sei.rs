//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataregistereditutt35sei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:21`
//! - Port note: Group 08 / Case 36

#[test]
#[ignore = "TODO: missing SEI parsing API in h265nal-sys"]
fn test_user_data_registered_itu_t_t35_sei() {
    // Test data for user data registered ITU-T T.35 SEI
    let _buffer = [
        0x04, 0x47, 0xb5, 0x00, 0x31, 0x47, 0x41, 0x39, 0x34, 0x03, 0x54, 0x00, 0xfc, 0x80, 0x80,
        0xfd, 0x80, 0x80, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
        0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
        0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
        0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xff,
    ];

    // TODO: Implement SEI parsing API in h265nal-sys
    // auto sei_message = H265SeiMessageParser::ParseSei(buffer, arraysize(buffer));
    // EXPECT_TRUE(sei_message != nullptr);
    // EXPECT_EQ(sei_message->payload_type, h265nal::SeiType::user_data_registered_itu_t_t35);
    // EXPECT_EQ(sei_message->payload_size, 71);
    // auto user_data_registered_itu_t_t35_sei = dynamic_cast<H265SeiUserDataRegisteredItuTT35Parser::H265SeiUserDataRegisteredItuTT35State*>(sei_message->payload_state.get());
    // EXPECT_TRUE(user_data_registered_itu_t_t35_sei != nullptr);
    // EXPECT_EQ(user_data_registered_itu_t_t35_sei->itu_t_t35_country_code, 181);
    // EXPECT_EQ(user_data_registered_itu_t_t35_sei->itu_t_t35_country_code_extension_byte, 0);

    todo!("SEI parsing API not yet implemented in h265nal-sys");
}
