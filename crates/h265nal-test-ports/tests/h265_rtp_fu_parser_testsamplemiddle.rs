//! References:
//! - Markdown: `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsamplemiddle.md`
//! - C++: `test/h265_rtp_fu_parser_unittest.cc:77`
//! - Port note: Group 06 / Case 26

#[ignore = "TODO: missing RTP FU parser API in h265nal-sys"]
#[test]
fn test_sample_middle() {
    // FU (Aggregation Packet) containing the middle of an IDR_W_RADL.
    // fuzzer::conv: data
    let buffer = [
        0x62, 0x01, 0x13, 0x8e, 0xaa, 0x12, 0xcc, 0xef, 0x6a, 0xf6, 0xb0, 0x7b, 0x7a, 0xbf, 0xea,
        0xf1, 0x3c, 0xa7, 0x20, 0xe8, 0x05, 0x9a, 0xfe, 0x6b,
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    // let rtp_fu = h265nal_sys::rtp_fu_parse(&buffer, &mut state).expect("ParseRtpFu failed");
    // assert!(rtp_fu.is_some());
    // let rtp_fu = rtp_fu.unwrap();
    // assert_eq!(rtp_fu.header.forbidden_zero_bit, 0);
    // assert_eq!(rtp_fu.header.nal_unit_type, h265nal_sys::NalUnitType::FU);
    // assert_eq!(rtp_fu.header.nuh_layer_id, 0);
    // assert_eq!(rtp_fu.header.nuh_temporal_id_plus1, 1);
    // assert_eq!(rtp_fu.s_bit, 0);
    // assert_eq!(rtp_fu.e_bit, 0);
    // assert_eq!(rtp_fu.fu_type, h265nal_sys::NalUnitType::IDR_W_RADL);
    assert!(true); // placeholder to make it compile
}
