//! References:
//! - Markdown: `docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsamplesingle.md`
//! - C++: `test/h265_rtp_parser_unittest.cc:22`
//! - Port note: Group 06 / Case 29

#[test]
fn test_sample_single() {
    // Single NAL Unit Packet (SPS for a 1280x736 camera capture).
    // fuzzer::conv: data
    let buffer = [
        0x42, 0x01, 0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x03, 0x00, 0x5d, 0xa0, 0x02, 0x80, 0x80, 0x2e, 0x1f, 0x13, 0x96, 0xbb, 0x93, 0x24, 0xbb,
        0x95, 0x82, 0x83, 0x03, 0x01, 0x76, 0x85, 0x09, 0x40,
    ];
    let mut bitstream_parser_state =
        h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    let rtp =
        h265nal_sys::rtp_parse(&buffer, &mut bitstream_parser_state).expect("ParseRtp failed");

    assert_eq!(rtp.packet_kind, h265nal_sys::RTP_PACKET_KIND_SINGLE);
    assert_eq!(rtp.forbidden_zero_bit, 0);
    assert_eq!(rtp.nal_unit_type, 33);
    assert_eq!(rtp.nuh_layer_id, 0);
    assert_eq!(rtp.nuh_temporal_id_plus1, 1);

    assert_eq!(rtp.has_payload_sps, 1);
    assert_eq!(rtp.payload_sps_pic_width_in_luma_samples, 1280);
    assert_eq!(rtp.payload_sps_pic_height_in_luma_samples, 736);
}
