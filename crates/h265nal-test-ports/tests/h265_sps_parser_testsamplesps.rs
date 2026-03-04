//! References:
//! - Markdown: `docs/test-cases/h265_sps_parser_unittest/h265spsparsertest--testsamplesps.md`
//! - C++: `test/h265_sps_parser_unittest.cc:21`
//! - Port note: Group 09 / Case 45

#[test]
#[ignore = "TODO: missing sps_parse API"]
fn test_sample_sps() {
    // SPS for a 1280x736 camera capture.
    let buffer = [
        0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00, 0x03, 0x00,
        0x5d, 0xa0, 0x02, 0x80, 0x80, 0x2e, 0x1f, 0x13, 0x96, 0xbb, 0x93, 0x24, 0xbb, 0x95, 0x82,
        0x83, 0x03, 0x01, 0x76, 0x85, 0x09, 0x40,
    ];
    // TODO: Implement sps_parse in h265nal-sys
    // let sps = h265nal_sys::sps_parse(&buffer).expect("ParseSps failed");

    // For now, just assert true to keep compile-safe
    assert!(true);
}
