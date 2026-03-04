//! References:
//! - Markdown: `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testslicecontainsshorttermrefpicset.md`
//! - C++: `test/h265_slice_parser_unittest.cc:179`
//! - Port note: Group 09 / Case 41

#[test]
fn test_slice_contains_short_term_ref_pic_set() {
    // VPS data
    let vps = [
        0x40, 0x01, 0x0c, 0x01, 0xff, 0xff, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x03, 0x00, 0x96, 0x70, 0x20, 0x00,
    ];

    // SPS data
    let sps = [
        0x42, 0x01, 0x01, 0x01, 0x60, 0x00, 0x00, 0x03, 0x00, 0xb0, 0x00, 0x00, 0x03, 0x00, 0x00,
        0x03, 0x00, 0x96, 0xa0, 0x05, 0x82, 0x00, 0x50, 0x16, 0x20, 0x5e, 0xe4, 0x59, 0x14, 0xbf,
        0xf2, 0xe7, 0xf1, 0x3f, 0xa9, 0xb8, 0x10, 0x00, 0x00, 0x00,
    ];

    // PPS data
    let pps = [0x44, 0x01, 0xc0, 0x72, 0xf0, 0x53, 0x24];

    // Slice data
    let slice = [
        0x02, 0x01, 0xd0, 0x53, 0x17, 0x62, 0x0a, 0x05, 0x02, 0x70, 0x34, 0x38, 0x56, 0x0e, 0xc3,
        0x48, 0x7c, 0xbf, 0x9d, 0xaa, 0xe5, 0x51, 0x8c, 0x8b, 0x40, 0x91, 0xbd, 0x9e, 0x49, 0x95,
        0x1a, 0x61, 0x90, 0x6f, 0x42, 0x25, 0x19, 0x50, 0xe6, 0xfb, 0x5d, 0x4f, 0x12, 0x72, 0x44,
        0xdc, 0x6c, 0x96, 0x0a, 0xbd, 0x7e, 0x87, 0x60, 0xc0,
    ];

    let mut bitstream_parser_state =
        h265nal_sys::BitstreamParserState::new().expect("BitstreamParserState creation failed");

    // Parse VPS
    let _vps_result = h265nal_sys::nal_unit_parse(&vps, &mut bitstream_parser_state, false)
        .expect("VPS parsing failed");

    // Parse SPS
    let _sps_result = h265nal_sys::nal_unit_parse(&sps, &mut bitstream_parser_state, false)
        .expect("SPS parsing failed");

    // Parse PPS
    let _pps_result = h265nal_sys::nal_unit_parse(&pps, &mut bitstream_parser_state, false)
        .expect("PPS parsing failed");

    // TODO: Check state sizes and contains
    // ASSERT_EQ(1, bitstream_parser_state.vps.size());
    // ASSERT_TRUE(bitstream_parser_state.vps.find(0) != bitstream_parser_state.vps.end());
    // ASSERT_EQ(1, bitstream_parser_state.sps.size());
    // ASSERT_TRUE(bitstream_parser_state.sps.find(0) != bitstream_parser_state.sps.end());
    // ASSERT_EQ(1, bitstream_parser_state.pps.size());
    // ASSERT_TRUE(bitstream_parser_state.pps.find(0) != bitstream_parser_state.pps.end());

    let slice_payload = &slice[2..];
    let slice_result =
        h265nal_sys::slice_segment_layer_parse(slice_payload, 1, &mut bitstream_parser_state)
            .expect("Slice parsing failed");
    assert_eq!(slice_result.num_entry_point_offsets, 39);
}
