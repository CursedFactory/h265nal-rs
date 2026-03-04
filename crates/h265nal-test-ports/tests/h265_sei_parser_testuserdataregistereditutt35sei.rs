//! References:
//! - Markdown: `docs/test-cases/h265_sei_parser_unittest/h265seiparsertest--testuserdataregistereditutt35sei.md`
//! - C++: `test/h265_sei_parser_unittest.cc:21`
//! - Port note: Group 08 / Case 36

#[test]
fn test_user_data_registered_itu_t_t35_sei() {
    // Test data for user data registered ITU-T T.35 SEI
    let buffer = [
        0x04, 0x47, 0xb5, 0x00, 0x31, 0x47, 0x41, 0x39, 0x34, 0x03, 0x54, 0x00, 0xfc, 0x80, 0x80,
        0xfd, 0x80, 0x80, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
        0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
        0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00,
        0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xfa, 0x00, 0x00, 0xff,
    ];

    let sei = h265nal_sys::sei_parse(&buffer).expect("ParseSei failed");
    assert_eq!(sei.payload_type, 4);
    assert_eq!(sei.payload_size, 71);
    assert_eq!(sei.has_user_data_registered_itu_t_t35, 1);
    assert_eq!(sei.user_data_registered_itu_t_t35_country_code, 181);
    assert_eq!(
        sei.user_data_registered_itu_t_t35_country_code_extension_byte,
        0
    );
    assert_eq!(sei.user_data_registered_itu_t_t35_payload_size, 70);
    assert_eq!(sei.user_data_registered_itu_t_t35_payload.len(), 70);
    assert_eq!(sei.user_data_registered_itu_t_t35_payload[0], 0x00);
    assert_eq!(sei.user_data_registered_itu_t_t35_payload[1], 0x31);
    assert_eq!(sei.has_unknown_payload, 0);
}
