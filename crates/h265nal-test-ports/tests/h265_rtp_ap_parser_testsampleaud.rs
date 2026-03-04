//! References:
//! - Markdown: `docs/test-cases/h265_rtp_ap_parser_unittest/h265rtpapparsertest--testsampleaud.md`
//! - C++: `test/h265_rtp_ap_parser_unittest.cc:21`
//! - Port note: Group 05 / Case 24

#[test]
fn test_sample_aud() {
    // AP (Aggregation Packet) containing VPS, PPS, SPS
    // fuzzer::conv: data
    let buffer = [
        // AP header
        0x60, 0x01, // NALU 1 size
        0x00, 0x17, // NALU 1 (VPS)
        0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x5d, 0xac, 0x09, // NALU 2 size
        0x00, 0x27, // NALU 2 (SPS)
        0x42, 0x01, 0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x03, 0x00, 0x5d, 0xa0, 0x02, 0x80, 0x80, 0x2e, 0x1f, 0x13, 0x96, 0xbb, 0x93, 0x24, 0xba,
        0x94, 0x82, 0x81, 0x01, 0x01, 0x76, 0x85, 0x09, 0x40, // NALU 3 size
        0x00, 0x0a, // NALU 3 (PPS)
        0x44, 0x01, 0xc0, 0xe2, 0x4f, 0x09, 0x41, 0xec, 0x10, 0x80,
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    let rtp = h265nal_sys::rtp_parse(&buffer, &mut state).expect("ParseRtp failed");

    assert_eq!(rtp.packet_kind, h265nal_sys::RTP_PACKET_KIND_AP);
    assert_eq!(rtp.forbidden_zero_bit, 0);
    assert_eq!(rtp.nal_unit_type, 48);
    assert_eq!(rtp.nuh_layer_id, 0);
    assert_eq!(rtp.nuh_temporal_id_plus1, 1);

    assert_eq!(rtp.ap_nal_unit_sizes_count, 3);
    assert_eq!(rtp.ap_nal_unit_headers_count, 3);
    assert_eq!(rtp.ap_nal_unit_payloads_count, 3);

    assert_eq!(rtp.ap_nal_unit_types, vec![32, 33, 34]);
}
