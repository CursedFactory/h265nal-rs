//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testcontentlightlevelinfosei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:143`
//! - Port note: Group 07 / Case 34

#[test]
fn test_content_light_level_info_sei() {
    // Test data for content light level info SEI
    let buffer = [
        0x90, 0x04, // payload_type = 144 (content_light_level_info)
        0x03, 0xe8, // max_content_light_level = 1000 cd/m^2
        0x01, 0x90, // max_pic_average_light_level = 400 cd/m^2
    ];

    let sei = h265nal_sys::sei_parse(&buffer).expect("ParseSei failed");
    assert_eq!(sei.payload_type, 144);
    assert_eq!(sei.payload_size, 4);
    assert_eq!(sei.has_content_light_level_info, 1);
    assert_eq!(sei.content_light_level_max_content_light_level, 1000);
    assert_eq!(sei.content_light_level_max_pic_average_light_level, 400);
}
