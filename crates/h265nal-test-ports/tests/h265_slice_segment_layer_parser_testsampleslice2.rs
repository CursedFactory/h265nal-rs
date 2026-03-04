//! References:
//! - Markdown: `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice2.md`
//! - C++: `test/h265_slice_parser_unittest.cc:77`

#[test]
fn test_sample_slice2() {
    let buffer = [
        0x26, 0x01, 0x20, 0xf3, 0xc3, 0x5c, 0xfd, 0xfe, 0xd6, 0x25, 0xbc, 0x0d, 0xb1, 0xfa, 0x81,
        0x63, 0xde, 0x0a, 0xc4, 0xa7, 0xea, 0x42, 0x89, 0xb6, 0x1e, 0xbb, 0x5e, 0x3f, 0xfd, 0x6c,
        0x8a, 0x2d,
    ];

    let mut state = h265nal_sys::BitstreamParserState::new().expect("state create failed");
    state.seed_vps(0).expect("seed_vps failed");
    state
        .seed_sps(0, 1, 1, 0, 2, 1280, 736)
        .expect("seed_sps failed");
    state.seed_pps(0).expect("seed_pps failed");

    let slice = h265nal_sys::slice_segment_layer_parse(&buffer, 19, &mut state)
        .expect("ParseSliceSegmentLayer failed");

    assert_eq!(slice.first_slice_segment_in_pic_flag, 0);
    assert_eq!(slice.no_output_of_prior_pics_flag, 0);
    assert_eq!(slice.slice_pic_parameter_set_id, 0);
    assert_eq!(slice.slice_segment_address, 192);
    assert_eq!(slice.slice_type, 3);
    assert_eq!(slice.slice_sao_luma_flag, 1);
    assert_eq!(slice.slice_sao_chroma_flag, 0);
    assert_eq!(slice.slice_qp_delta, 15);
}
