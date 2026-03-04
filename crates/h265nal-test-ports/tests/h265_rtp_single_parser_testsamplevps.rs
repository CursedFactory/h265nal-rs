//! References:
//! - Markdown: `docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testsamplevps.md`
//! - C++: `test/h265_rtp_single_parser_unittest.cc:24`
//! - Port note: Group 07 / Case 31

#[test]
#[ignore = "TODO: missing h265nal_rtp_single_parse function in h265nal-sys"]
fn test_sample_vps() {
    // VPS for a 1280x720 camera capture.
    // fuzzer::conv: data
    let _buffer = [
        0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x5d, 0xac, 0x59, 0x00,
    ];
    // fuzzer::conv: begin
    let _state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    // TODO: let rtp_single = h265nal_rtp_single_parse(&buffer, &mut state).expect("ParseRtpSingle failed");
    // TODO: assert!(rtp_single.is_some());

    // check the header
    // TODO: let header = &rtp_single.as_ref().unwrap().nal_unit_header;
    // TODO: assert_eq!(header.forbidden_zero_bit, 0);
    // TODO: assert_eq!(header.nal_unit_type, NalUnitType::VPS_NUT as u32);
    // TODO: assert_eq!(header.nuh_layer_id, 0);
    // TODO: assert_eq!(header.nuh_temporal_id_plus1, 1);
}
