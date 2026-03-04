//! References:
//! - Markdown: `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testspsbadheight.md`
//! - C++: `test/h265_sps_parser_unittest.cc:176`
//! - Port note: Group 10 / Case 46

#[test]
fn test_sps_bad_height() {
    // SPS for a 8x7 camera capture (7 does not divide 8)
    // fuzzer::conv: data
    let buffer = [
        0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00, 0x03, 0x00,
        0x5d, 0xa0, 0x02, 0x80, 0x80, 0x1e, 0x1f, 0x13, 0x96, 0xbb, 0x93, 0x24, 0xbb, 0x95, 0x82,
        0x83, 0x03, 0x01, 0x76, 0x85, 0x09, 0x40,
    ];
    let result = h265nal_sys::sps_parse(&buffer);

    // The parse should fail, returning an error (equivalent to sps == nullptr in C++)
    assert!(result.is_err());
}
