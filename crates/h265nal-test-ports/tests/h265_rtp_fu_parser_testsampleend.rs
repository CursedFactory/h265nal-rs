//! References:
//! - Markdown: `docs/test-cases/h265_rtp_fu_parser_unittest/h265rtpfuparsertest--testsampleend.md`
//! - C++: `test/h265_rtp_fu_parser_unittest.cc:104`
//! - Port note: Group 05 / Case 25

#[ignore = "TODO: missing RTP FU parser API in h265nal-sys"]
#[test]
fn test_sample_end() {
    // TODO: implement when h265nal_sys::rtp_fu_parse is available
    let buffer = [
        0x62, 0x01, 0x53, 0x85, 0xfe, 0xde, 0xe8, 0x9c, 0x2f, 0xd5, 0xed, 0xb9, 0x2c, 0xec, 0x8f,
        0x08, 0x5b, 0x95, 0x02, 0x79, 0xc3, 0xb7, 0x47, 0x16,
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
    // assert_eq!(rtp_fu.e_bit, 1);
    // assert_eq!(rtp_fu.fu_type, h265nal_sys::NalUnitType::IDR_W_RADL);
    assert!(true); // placeholder to make it compile
}
