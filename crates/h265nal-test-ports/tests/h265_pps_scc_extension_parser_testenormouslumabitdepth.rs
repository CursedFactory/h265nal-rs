//! References:
//! - Markdown: `docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testenormouslumabitdepth.md`
//! - C++: `test/h265_pps_scc_extension_parser_unittest.cc:40`
//! - Port note: Group 04 / Case 18

#[test]
fn test_enormous_luma_bit_depth() {
    let buffer = [
        0xa8u8, 0x00, 0x00, 0x00, 0x03, 0x03, 0xff, 0xff, 0xff, 0xe4, 0x00, 0x00,
    ];
    let pps_scc_extension = h265nal_sys::pps_scc_extension_parse(&buffer);
    assert_eq!(pps_scc_extension, Err(h265nal_sys::Error::ParseFailure));
}
