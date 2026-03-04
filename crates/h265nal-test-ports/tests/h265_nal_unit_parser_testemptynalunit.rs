//! References:
//! - Markdown: `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitparsertest--testemptynalunit.md`
//! - C++: `test/h265_nal_unit_parser_unittest.cc:126`
//! - Port note: Group 03 / Case 14

#[test]
fn test_empty_nal_unit() {
    let buffer = [0u8];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    let nal_unit = h265nal_sys::nal_unit_parse(&buffer[..0], &mut state, false);
    assert_eq!(nal_unit, Err(h265nal_sys::Error::ParseFailure));
}
