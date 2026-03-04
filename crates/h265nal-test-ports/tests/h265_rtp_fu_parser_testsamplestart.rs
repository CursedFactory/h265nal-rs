//! References:
//! - Markdown: `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplestart.md`
//! - C++: `test/h265_rtp_fu_parser_unittest.cc:21`
//! - Port note: Group 06 / Case 27

#[test]
#[ignore = "TODO: missing h265nal_rtp_fu_parse function in h265nal-sys"]
fn test_sample_start() {
    // FU (Aggregation Packet) containing the start of an IDR_W_RADL.
    // fuzzer::conv: data
    let buffer = [
        0x62, 0x01, 0x93, 0xaf, 0x0d, 0x70, 0xfd, 0xf4, 0x6e, 0xf0, 0x3c, 0x7e, 0x63, 0xc8, 0x15,
        0xf5, 0xf7, 0x6e, 0x52, 0x0f, 0xd3, 0xb5, 0x44, 0x61, 0x58, 0x24, 0x68, 0xe0,
    ];
    // fuzzer::conv: begin
    // get some mock state
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    // TODO: let vps = ... (set up VPS in state)
    // TODO: let sps = ... (set up SPS in state with sample_adaptive_offset_enabled_flag = 1, chroma_format_idc = 1)
    // TODO: let pps = ... (set up PPS in state)

    // TODO: let rtp_fu = h265nal_rtp_fu_parse(&buffer, &mut state).expect("ParseRtpFu failed");
    // fuzzer::conv: end

    // TODO: assert!(rtp_fu.is_some());

    // TODO: check the common header
    // TODO: let header = &rtp_fu.as_ref().unwrap().header;
    // TODO: assert_eq!(header.forbidden_zero_bit, 0);
    // TODO: assert_eq!(header.nal_unit_type, 49); // NalUnitType::FU
    // TODO: assert_eq!(header.nuh_layer_id, 0);
    // TODO: assert_eq!(header.nuh_temporal_id_plus1, 1);

    // TODO: check the fu header
    // TODO: assert_eq!(rtp_fu.as_ref().unwrap().s_bit, 1);
    // TODO: assert_eq!(rtp_fu.as_ref().unwrap().e_bit, 0);
    // TODO: assert_eq!(rtp_fu.as_ref().unwrap().fu_type, 19); // NalUnitType::IDR_W_RADL

    // TODO: check some values
    // TODO: let nal_unit_payload = &rtp_fu.as_ref().unwrap().nal_unit_payload;
    // TODO: assert!(nal_unit_payload.is_some());

    // TODO: let slice_segment_layer = &nal_unit_payload.as_ref().unwrap().slice_segment_layer;
    // TODO: assert!(slice_segment_layer.is_some());

    // TODO: let slice_segment_header = &slice_segment_layer.as_ref().unwrap().slice_segment_header;
    // TODO: assert!(slice_segment_header.is_some());

    // TODO: assert_eq!(slice_segment_header.as_ref().unwrap().nal_unit_type, 19); // NalUnitType::IDR_W_RADL
    // TODO: assert_eq!(slice_segment_header.as_ref().unwrap().first_slice_segment_in_pic_flag, 1);
    // TODO: assert_eq!(slice_segment_header.as_ref().unwrap().no_output_of_prior_pics_flag, 0);
    // TODO: assert_eq!(slice_segment_header.as_ref().unwrap().slice_pic_parameter_set_id, 0);
    // TODO: assert_eq!(slice_segment_header.as_ref().unwrap().slice_type, 2);
}
