//! References:
//! - Markdown: `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsampleend.md`
//! - C++: `test/h265_rtp_fu_parser_unittest.cc:104`
//! - Port note: Group 05 / Case 25

#[test]
fn test_sample_end() {
    let buffer = [
        0x62, 0x01, 0x53, 0x85, 0xfe, 0xde, 0xe8, 0x9c, 0x2f, 0xd5, 0xed, 0xb9, 0x2c, 0xec, 0x8f,
        0x08, 0x5b, 0x95, 0x02, 0x79, 0xc3, 0xb7, 0x47, 0x16,
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    let rtp = h265nal_sys::rtp_parse(&buffer, &mut state).expect("ParseRtp failed");

    assert_eq!(rtp.packet_kind, h265nal_sys::RTP_PACKET_KIND_FU);
    assert_eq!(rtp.forbidden_zero_bit, 0);
    assert_eq!(rtp.nal_unit_type, 49);
    assert_eq!(rtp.nuh_layer_id, 0);
    assert_eq!(rtp.nuh_temporal_id_plus1, 1);
    assert_eq!(rtp.fu_s_bit, 0);
    assert_eq!(rtp.fu_e_bit, 1);
    assert_eq!(rtp.fu_type, 19);
    assert_eq!(rtp.fu_has_nal_unit_payload, 0);
}
