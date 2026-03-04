//! References:
//! - Markdown: `docs/test-cases/h265_nal_unit_parser_unittest/h265nalunitheaderparsertest--verifparsenalutype.md`
//! - C++: `test/h265_nal_unit_parser_unittest.cc:143`
//! - Port note: Group 03 / Case 13

mod support;

#[test]
fn verif_parse_nalu_type() {
    let buffer = [
        0x20, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x5d, 0xac, 0x59, 0x00,
    ];
    let nal_unit_type =
        h265nal_sys::nal_unit_header_get_nal_unit_type(&buffer).expect("GetNalUnitType failed");
    assert_eq!(support::BLA_W_LP, nal_unit_type);
}
