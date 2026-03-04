//! References:
//! - Markdown: `docs/test-cases/h265_pps_scc_extension_parser_unittest/h265ppssccextensionparsertest--testenormouslumabitdepth.md`
//! - C++: `test/h265_pps_scc_extension_parser_unittest.cc:40`
//! - Port note: Group 04 / Case 18

#[test]
#[ignore = "TODO: missing pps_scc_extension parser API"]
fn test_enormous_luma_bit_depth() {
    let buffer = [
        0xa8u8, 0x00, 0x00, 0x00, 0x03, 0x03, 0xff, 0xff, 0xff, 0xe4, 0x00, 0x00,
    ];
    // TODO: Implement pps_scc_extension parser
    // let pps_scc_extension = h265nal_sys::parse_pps_scc_extension(&buffer).expect("ParsePpsSccExtension failed");
    // assert!(pps_scc_extension.is_none());
}
