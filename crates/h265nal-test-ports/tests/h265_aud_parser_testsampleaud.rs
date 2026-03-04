//! References:
//! - Markdown: `docs/test-cases/h265_aud_parser_unittest/h265audparsertest--testsampleaud.md`
//! - C++: `test/h265_aud_parser_unittest.cc:21`
//! - Port note: Group 01 / Case 01

#[test]
fn test_sample_aud() {
    let buffer = [0xffu8];
    let pic_type = h265nal_sys::aud_parse_pic_type(&buffer).expect("ParseAud failed");
    assert_eq!(pic_type, 7);
}
