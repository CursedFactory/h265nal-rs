//! References:
//! - Markdown: `docs/test-cases/h265_rtp_ap_parser_unittest/h265rtpapparsertest--testsampleaud.md`
//! - C++: `test/h265_rtp_ap_parser_unittest.cc:21`
//! - Port note: Group 05 / Case 24

#[test]
#[ignore = "TODO: missing h265nal_rtp_ap_parse function in h265nal-sys"]
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
    // fuzzer::conv: begin
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    // TODO: rtp_ap = h265nal_rtp_ap_parse(&buffer, &mut state).expect("ParseRtpAp failed");
    // fuzzer::conv: end

    // TODO: assert!(rtp_ap.is_some());

    // TODO: check the common header
    // TODO: let header = &rtp_ap.as_ref().unwrap().header;
    // TODO: assert_eq!(header.forbidden_zero_bit, 0);
    // TODO: assert_eq!(header.nal_unit_type, NalUnitType::AP as u32);
    // TODO: assert_eq!(header.nuh_layer_id, 0);
    // TODO: assert_eq!(header.nuh_temporal_id_plus1, 1);

    // TODO: check there are 3 valid NAL units
    // TODO: assert_eq!(rtp_ap.as_ref().unwrap().nal_unit_sizes.len(), 3);
    // TODO: assert_eq!(rtp_ap.as_ref().unwrap().nal_unit_headers.len(), 3);
    // TODO: assert_eq!(rtp_ap.as_ref().unwrap().nal_unit_payloads.len(), 3);

    // TODO: check the types
    // TODO: assert_eq!(rtp_ap.as_ref().unwrap().nal_unit_headers[0].nal_unit_type, NalUnitType::VPS_NUT as u32);
    // TODO: assert_eq!(rtp_ap.as_ref().unwrap().nal_unit_headers[1].nal_unit_type, NalUnitType::SPS_NUT as u32);
    // TODO: assert_eq!(rtp_ap.as_ref().unwrap().nal_unit_headers[2].nal_unit_type, NalUnitType::PPS_NUT as u32);

    // TODO: check the parser state
}
