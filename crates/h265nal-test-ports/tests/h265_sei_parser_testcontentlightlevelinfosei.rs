//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testcontentlightlevelinfosei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:143`
//! - Port note: Group 07 / Case 34

#[test]
#[ignore = "TODO: missing SEI parsing API in h265nal-sys"]
fn test_content_light_level_info_sei() {
    // Test data for content light level info SEI
    let _buffer = [
        0x90, 0x04, // payload_type = 144 (content_light_level_info)
        0x03, 0xe8, // max_content_light_level = 1000 cd/m^2
        0x01, 0x90, // max_pic_average_light_level = 400 cd/m^2
    ];

    // TODO: Implement SEI parsing API in h265nal-sys
    // auto sei_message = H265SeiMessageParser::ParseSei(buffer, arraysize(buffer));
    // EXPECT_TRUE(sei_message != nullptr);
    // EXPECT_EQ(sei_message->payload_type, h265nal::SeiType::content_light_level_info);
    // EXPECT_EQ(sei_message->payload_size, 4);
    // auto content_light_sei = dynamic_cast<H265SeiContentLightLevelInfoParser::H265SeiContentLightLevelInfoState*>(sei_message->payload_state.get());
    // EXPECT_TRUE(content_light_sei != nullptr);
    // EXPECT_EQ(content_light_sei->max_content_light_level, 1000);
    // EXPECT_EQ(content_light_sei->max_pic_average_light_level, 400);

    todo!("SEI parsing API not yet implemented in h265nal-sys");
}
