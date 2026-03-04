//! References:
//! - Markdown: `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplestart.md`
//! - C++: `test/h265_rtp_fu_parser_unittest.cc:21`
//! - Port note: Group 06 / Case 27

#[test]
fn test_sample_start() {
    // FU (Aggregation Packet) containing the start of an IDR_W_RADL.
    // fuzzer::conv: data
    let buffer = [
        0x62, 0x01, 0x93, 0xaf, 0x0d, 0x70, 0xfd, 0xf4, 0x6e, 0xf0, 0x3c, 0x7e, 0x63, 0xc8, 0x15,
        0xf5, 0xf7, 0x6e, 0x52, 0x0f, 0xd3, 0xb5, 0x44, 0x61, 0x58, 0x24, 0x68, 0xe0,
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    state.seed_vps(0).expect("seed_vps failed");
    state
        .seed_sps(0, 1, 1, 0, 0, 0, 0)
        .expect("seed_sps failed");
    state.seed_pps(0).expect("seed_pps failed");

    let rtp = h265nal_sys::rtp_parse(&buffer, &mut state).expect("ParseRtp failed");

    assert_eq!(rtp.packet_kind, h265nal_sys::RTP_PACKET_KIND_FU);
    assert_eq!(rtp.forbidden_zero_bit, 0);
    assert_eq!(rtp.nal_unit_type, 49);
    assert_eq!(rtp.nuh_layer_id, 0);
    assert_eq!(rtp.nuh_temporal_id_plus1, 1);

    assert_eq!(rtp.fu_s_bit, 1);
    assert_eq!(rtp.fu_e_bit, 0);
    assert_eq!(rtp.fu_type, 19);
    assert_eq!(rtp.fu_has_nal_unit_payload, 1);

    assert_eq!(rtp.has_payload_slice_segment_header, 1);
    assert_eq!(rtp.payload_slice_nal_unit_type, 19);
    assert_eq!(rtp.payload_slice_first_slice_segment_in_pic_flag, 1);
    assert_eq!(rtp.payload_slice_no_output_of_prior_pics_flag, 0);
    assert_eq!(rtp.payload_slice_pic_parameter_set_id, 0);
    assert_eq!(rtp.payload_slice_type, 2);
}
