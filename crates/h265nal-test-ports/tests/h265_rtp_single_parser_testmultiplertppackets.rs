//! References:
//! - Markdown: `docs/test-cases/h265_rtp_single_parser_unittest/h265rtpsingleparsertest--testmultiplertppackets.md`
//! - C++: `test/h265_rtp_single_parser_unittest.cc:45`
//! - Port note: Group 06 / Case 30

#[test]
fn test_multiple_rtp_packets() {
    let vbuffer = vec![
        // VPS for a 1280x720 camera capture.
        vec![
            0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00,
            0x00, 0x03, 0x00, 0x00, 0x03, 0x00, 0x5d, 0xac, 0x59, 0x00,
        ],
        // SPS
        vec![
            0x42, 0x01, 0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00,
            0x00, 0x03, 0x00, 0x5d, 0xa0, 0x02, 0x80, 0x80, 0x2e, 0x1f, 0x13, 0x96, 0xbb, 0x93,
            0x24, 0xbb, 0x95, 0x82, 0x83, 0x03, 0x01, 0x76, 0x85, 0x09, 0x40, 0x00,
        ],
        // PPS
        vec![0x44, 0x01, 0xc0, 0xf3, 0xc0, 0x02, 0x10, 0x00],
        // slice
        vec![
            0x26, 0x01, 0xaf, 0x09, 0x40, 0xf3, 0xb8, 0xd5, 0x39, 0xba, 0x1f, 0xe4, 0xa6, 0x08,
            0x5c, 0x6e, 0xb1, 0x8f, 0x00, 0x38, 0xf1, 0xa6, 0xfc, 0xf1, 0x40, 0x04, 0x3a, 0x86,
            0xcb, 0x90, 0x74, 0xce, 0xf0, 0x46, 0x61, 0x93, 0x72, 0xd6, 0xfc, 0x35, 0xe3, 0xc5,
            0x6f, 0x0a, 0xc4, 0x9e, 0x27, 0xc4, 0xdb, 0xe3, 0xfb, 0x38, 0x98, 0xd0, 0x8b, 0xd5,
            0xb9, 0xb9, 0x15, 0xb4, 0x92, 0x49, 0x97, 0xe5, 0x3d, 0x36, 0x4d, 0x45, 0x32, 0x5c,
            0xe6, 0x89, 0x53, 0x76, 0xce, 0xbb, 0x83, 0xa1, 0x27, 0x35, 0xfb, 0xf3, 0xc7, 0xd4,
            0x85, 0x32, 0x37, 0x94, 0x09, 0xec, 0x10,
        ],
    ];
    let mut state = h265nal_sys::BitstreamParserState::new().expect("Failed to create state");
    for (i, buffer) in vbuffer.iter().enumerate() {
        let rtp = h265nal_sys::rtp_parse(buffer, &mut state).expect("ParseRtp failed");
        assert_eq!(rtp.packet_kind, h265nal_sys::RTP_PACKET_KIND_SINGLE);

        if i == 0 {
            assert_eq!(rtp.forbidden_zero_bit, 0);
            assert_eq!(rtp.nal_unit_type, 32);
            assert_eq!(rtp.nuh_layer_id, 0);
            assert_eq!(rtp.nuh_temporal_id_plus1, 1);
        } else if i == 1 {
            assert_eq!(rtp.forbidden_zero_bit, 0);
            assert_eq!(rtp.nal_unit_type, 33);
            assert_eq!(rtp.nuh_layer_id, 0);
            assert_eq!(rtp.nuh_temporal_id_plus1, 1);
            assert_eq!(rtp.has_payload_sps, 1);
            assert_eq!(rtp.payload_sps_pic_width_in_luma_samples, 1280);
            assert_eq!(rtp.payload_sps_pic_height_in_luma_samples, 736);
        } else if i == 2 {
            assert_eq!(rtp.forbidden_zero_bit, 0);
            assert_eq!(rtp.nal_unit_type, 34);
            assert_eq!(rtp.nuh_layer_id, 0);
            assert_eq!(rtp.nuh_temporal_id_plus1, 1);
            assert_eq!(rtp.has_payload_pps, 1);
            assert_eq!(rtp.payload_pps_init_qp_minus26, 0);
        } else if i == 3 {
            assert_eq!(rtp.forbidden_zero_bit, 0);
            assert_eq!(rtp.nal_unit_type, 19);
            assert_eq!(rtp.nuh_layer_id, 0);
            assert_eq!(rtp.nuh_temporal_id_plus1, 1);
            assert_eq!(rtp.has_payload_slice_segment_header, 1);
            assert_eq!(rtp.payload_slice_qp_delta, 9);
            assert_eq!(rtp.payload_slice_pic_parameter_set_id, 0);
        }
    }
}
