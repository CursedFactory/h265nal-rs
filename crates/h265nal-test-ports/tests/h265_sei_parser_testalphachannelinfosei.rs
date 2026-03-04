//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testalphachannelinfosei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:76`
//! - Port note: Group 07 / Case 33

#[test]
#[ignore = "TODO: missing SEI parsing APIs in h265nal-sys"]
fn test_alpha_channel_info_sei() {
    // TODO: Implement once SEI parsing APIs are available in h265nal-sys
    // Equivalent to H265SeiMessageParser::ParseSei
    // const uint8_t buffer[] = {0xa5, 0x04, 0x00, 0x00, 0x7f, 0x90, 0x80};
    // let sei_message = h265nal_sys::sei_parse_message(&buffer).expect("ParseSei failed");
    // assert!(sei_message.is_some());
    // let alpha_channel_info_sei = sei_message.as_ref().unwrap().alpha_channel_info_sei.as_ref().unwrap();
    // assert_eq!(alpha_channel_info_sei.alpha_channel_cancel_flag, 0);
    // assert_eq!(alpha_channel_info_sei.alpha_channel_use_idc, 0);
    // assert_eq!(alpha_channel_info_sei.alpha_channel_bit_depth_minus8, 0);
    // assert_eq!(alpha_channel_info_sei.alpha_transparent_value, 0);
    // assert_eq!(alpha_channel_info_sei.alpha_opaque_value, 255);
    // assert_eq!(alpha_channel_info_sei.alpha_channel_incr_flag, 0);
    // assert_eq!(alpha_channel_info_sei.alpha_channel_clip_flag, 0);
}
