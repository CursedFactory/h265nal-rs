//! References:
//! - Markdown: `docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testsamplevps.md`
//! - C++: `test/h265_rtp_single_parser_unittest.cc:24`
//! - Port note: Group 07 / Case 31

#[test]
fn test_sample_vps() {
    // VPS for a 1280x720 camera capture.
    // fuzzer::conv: data
    let buffer = [
        0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x5d, 0xac, 0x59, 0x00,
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    let rtp = h265nal_sys::rtp_parse(&buffer, &mut state).expect("ParseRtp failed");

    assert_eq!(rtp.packet_kind, h265nal_sys::RTP_PACKET_KIND_SINGLE);
    assert_eq!(rtp.forbidden_zero_bit, 0);
    assert_eq!(rtp.nal_unit_type, 32);
    assert_eq!(rtp.nuh_layer_id, 0);
    assert_eq!(rtp.nuh_temporal_id_plus1, 1);
}
