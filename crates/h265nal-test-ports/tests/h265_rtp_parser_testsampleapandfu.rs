//! References:
//! - Markdown: `docs/test-cases/h265_rtp_parser_unittest/h265rtpparsertest--testsampleapandfu.md`
//! - C++: `test/h265_rtp_parser_unittest.cc:59`

#[test]
#[ignore = "TODO: missing h265nal_rtp_parse function in h265nal-sys"]
fn test_sample_ap_and_fu() {
    // AP (Aggregation Packet) containing VPS, PPS, SPS
    // fuzzer::conv: data
    let buffer1 = [
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
    // TODO: let rtp = h265nal_rtp_parse(&buffer1, &mut state).expect("ParseRtp failed");

    // TODO: assert!(rtp.is_some());

    // check the common header
    // TODO: let ap_header = &rtp.as_ref().unwrap().nal_unit_header;
    // TODO: assert_eq!(ap_header.forbidden_zero_bit, 0);
    // TODO: assert_eq!(ap_header.nal_unit_type, NalUnitType::AP as u32);
    // TODO: assert_eq!(ap_header.nuh_layer_id, 0);
    // TODO: assert_eq!(ap_header.nuh_temporal_id_plus1, 1);

    // check there are 3 valid NAL units
    // TODO: let rtp_ap = &rtp.as_ref().unwrap().rtp_ap;
    // TODO: assert_eq!(rtp_ap.nal_unit_sizes.len(), 3);
    // TODO: assert_eq!(rtp_ap.nal_unit_headers.len(), 3);
    // TODO: assert_eq!(rtp_ap.nal_unit_payloads.len(), 3);

    // check the types
    // TODO: assert_eq!(rtp_ap.nal_unit_headers[0].nal_unit_type, NalUnitType::VPS_NUT as u32);
    // TODO: assert_eq!(rtp_ap.nal_unit_headers[1].nal_unit_type, NalUnitType::SPS_NUT as u32);
    // TODO: assert_eq!(rtp_ap.nal_unit_headers[2].nal_unit_type, NalUnitType::PPS_NUT as u32);

    // check some values
    // TODO: let sps = &rtp_ap.nal_unit_payloads[1].sps;
    // TODO: assert_eq!(sps.pic_width_in_luma_samples, 1280);
    // TODO: assert_eq!(sps.pic_height_in_luma_samples, 736);
    // TODO: let sps_id = sps.sps_seq_parameter_set_id;
    // TODO: assert_eq!(state.sps[sps_id as usize].pic_width_in_luma_samples, 1280);
    // TODO: assert_eq!(state.sps[sps_id as usize].pic_height_in_luma_samples, 736);
    // TODO: let pps = &rtp_ap.nal_unit_payloads[2].pps;
    // TODO: assert_eq!(pps.init_qp_minus26, -4);
    // TODO: let pps_id = pps.pps_pic_parameter_set_id;
    // TODO: assert_eq!(state.pps[pps_id as usize].init_qp_minus26, -4);

    // FU (Fragmentation Unit) containing the start of an IDR_W_RADL.
    let buffer2 = [
        0x62, 0x01, 0x93, 0xaf, 0x0d, 0x70, 0xfd, 0xf4, 0x6e, 0xf0, 0x3c, 0x7e, 0x63, 0xc8, 0x15,
        0xf5, 0xf7, 0x6e, 0x52, 0x0f, 0xd3, 0xb5, 0x44, 0x61, 0x58, 0x24, 0x68, 0xe0,
    ];
    // TODO: let rtp = h265nal_rtp_parse(&buffer2, &mut state).expect("ParseRtp failed");
    // TODO: assert!(rtp.is_some());

    // check the main header
    // TODO: let fu_header = &rtp.as_ref().unwrap().nal_unit_header;
    // TODO: assert_eq!(fu_header.forbidden_zero_bit, 0);
    // TODO: assert_eq!(fu_header.nal_unit_type, NalUnitType::FU as u32);
    // TODO: assert_eq!(fu_header.nuh_layer_id, 0);
    // TODO: assert_eq!(fu_header.nuh_temporal_id_plus1, 1);

    // check the fu header
    // TODO: let rtp_fu = &rtp.as_ref().unwrap().rtp_fu;
    // TODO: assert_eq!(rtp_fu.s_bit, 1);
    // TODO: assert_eq!(rtp_fu.e_bit, 0);
    // TODO: assert_eq!(rtp_fu.fu_type, NalUnitType::IDR_W_RADL as u32);

    // check some values
    // TODO: let slice_segment_header = &rtp_fu.nal_unit_payload.slice_segment_layer.slice_segment_header;
    // TODO: assert_eq!(slice_segment_header.slice_qp_delta, 13);

    // TODO: let slice_qpy = h265nal_sys::utils_get_slice_qp_y(&buffer2, &mut state).expect("GetSliceQpY failed");
    // TODO: assert_eq!(slice_qpy, 35);
}
