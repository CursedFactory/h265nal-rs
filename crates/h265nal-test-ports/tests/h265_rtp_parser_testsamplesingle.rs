//! References:
//! - Markdown: `docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsamplesingle.md`
//! - C++: `test/h265_rtp_parser_unittest.cc:22`
//! - Port note: Group 06 / Case 29

#[test]
#[ignore = "TODO: missing h265nal_rtp_parse function in h265nal-sys"]
fn test_sample_single() {
    // Single NAL Unit Packet (SPS for a 1280x736 camera capture).
    // fuzzer::conv: data
    let _buffer = [
        0x42, 0x01, 0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x03, 0x00, 0x5d, 0xa0, 0x02, 0x80, 0x80, 0x2e, 0x1f, 0x13, 0x96, 0xbb, 0x93, 0x24, 0xbb,
        0x95, 0x82, 0x83, 0x03, 0x01, 0x76, 0x85, 0x09, 0x40,
    ];
    // fuzzer::conv: begin
    let _bitstream_parser_state =
        h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    // TODO: let rtp = h265nal_rtp_parse(&buffer, &mut bitstream_parser_state).expect("ParseRtp failed");
    // fuzzer::conv: end

    // TODO: assert!(rtp.is_some());

    // check the header
    // TODO: let header = &rtp.as_ref().unwrap().nal_unit_header;
    // TODO: assert_eq!(header.forbidden_zero_bit, 0);
    // TODO: assert_eq!(header.nal_unit_type, h265nal_sys::NalUnitType::SPS_NUT as u32);
    // TODO: assert_eq!(header.nuh_layer_id, 0);
    // TODO: assert_eq!(header.nuh_temporal_id_plus1, 1);

    // check some values
    // TODO: let rtp_single = &rtp.as_ref().unwrap().rtp_single;
    // TODO: let sps = &rtp_single.nal_unit_payload.sps;
    // TODO: assert_eq!(sps.pic_width_in_luma_samples, 1280);
    // TODO: assert_eq!(sps.pic_height_in_luma_samples, 736);
    // TODO: let sps_id = sps.sps_seq_parameter_set_id;
    // TODO: assert_eq!(bitstream_parser_state.sps[sps_id as usize].pic_width_in_luma_samples, 1280);
    // TODO: assert_eq!(bitstream_parser_state.sps[sps_id as usize].pic_height_in_luma_samples, 736);
}
