//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testalphachannelinfosei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:76`
//! - Port note: Group 07 / Case 33

#[test]
fn test_alpha_channel_info_sei() {
    let buffer = [0xa5, 0x04, 0x00, 0x00, 0x7f, 0x90, 0x80];

    let sei = h265nal_sys::sei_parse(&buffer).expect("ParseSei failed");
    assert_eq!(sei.payload_type, 165);
    assert_eq!(sei.payload_size, 4);
    assert_eq!(sei.has_alpha_channel_info, 1);
    assert_eq!(sei.alpha_channel_cancel_flag, 0);
    assert_eq!(sei.alpha_channel_use_idc, 0);
    assert_eq!(sei.alpha_channel_bit_depth_minus8, 0);
    assert_eq!(sei.alpha_transparent_value, 0);
    assert_eq!(sei.alpha_opaque_value, 255);
    assert_eq!(sei.alpha_channel_incr_flag, 0);
    assert_eq!(sei.alpha_channel_clip_flag, 0);
}
