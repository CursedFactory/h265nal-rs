//! References:
//! - Markdown: `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice.md`
//! - C++: `test/h265_slice_parser_unittest.cc:26`
//! - Port note: Group 08 / Case 38

#[test]
fn test_sample_slice() {
    // fuzzer::conv: data
    let buffer = [
        0xaf, 0x09, 0x40, 0xf3, 0xb8, 0xd5, 0x39, 0xba, 0x1f, 0xe4, 0xa6, 0x08, 0x5c, 0x6e, 0xb1,
        0x8f, 0x00, 0x38, 0xf1, 0xa6, 0xfc, 0xf1, 0x40, 0x04, 0x3a, 0x86, 0xcb, 0x90, 0x74, 0xce,
        0xf0, 0x46, 0x61, 0x93, 0x72, 0xd6, 0xfc, 0x35, 0xe3, 0xc5,
    ];

    let mut state = h265nal_sys::BitstreamParserState::new().expect("state create failed");
    state.seed_vps(0).expect("seed_vps failed");
    state
        .seed_sps(0, 1, 1, 0, 0, 0, 0)
        .expect("seed_sps failed");
    state.seed_pps(0).expect("seed_pps failed");

    let slice = h265nal_sys::slice_segment_layer_parse(&buffer, 19, &mut state)
        .expect("ParseSliceSegmentLayer failed");

    assert_eq!(slice.has_slice_segment_header, 1);
    assert_eq!(slice.first_slice_segment_in_pic_flag, 1);
    assert_eq!(slice.no_output_of_prior_pics_flag, 0);
    assert_eq!(slice.slice_pic_parameter_set_id, 0);
    assert_eq!(slice.slice_type, 2);
    assert_eq!(slice.slice_sao_luma_flag, 1);
    assert_eq!(slice.slice_sao_chroma_flag, 1);
    assert_eq!(slice.slice_qp_delta, 9);
}
