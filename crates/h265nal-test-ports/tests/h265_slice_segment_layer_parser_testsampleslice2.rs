//! References:
//! - Markdown: `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice2.md`
//! - C++: `test/h265_slice_parser_unittest.cc:77`

#[test]
#[ignore = "TODO: missing slice_segment_layer_parse API"]
fn test_sample_slice2() {
    // TODO: const buffer = [
    // TODO:     0x26, 0x01, 0x20, 0xf3, 0xc3, 0x5c, 0xfd, 0xfe,
    // TODO:     0xd6, 0x25, 0xbc, 0x0d, 0xb1, 0xfa, 0x81, 0x63,
    // TODO:     0xde, 0x0a, 0xc4, 0xa7, 0xea, 0x42, 0x89, 0xb6,
    // TODO:     0x1e, 0xbb, 0x5e, 0x3f, 0xfd, 0x6c, 0x8a, 0x2d
    // TODO: ];

    // TODO: let mut state = h265nal_sys::BitstreamParserState::new().expect("BitstreamParserState creation failed");

    // TODO: // Set up VPS, SPS, PPS in state as per C++ test

    // TODO: let slice_segment_layer = h265nal_sys::slice_segment_layer_parse(&buffer, NalUnitType::IDR_W_RADL, &mut state).expect("ParseSliceSegmentLayer failed");

    // TODO: let slice_segment_header = &slice_segment_layer.slice_segment_header;

    // TODO: assert_eq!(slice_segment_header.first_slice_segment_in_pic_flag, 0);
    // TODO: assert_eq!(slice_segment_header.no_output_of_prior_pics_flag, 0);
    // TODO: assert_eq!(slice_segment_header.slice_pic_parameter_set_id, 0);
    // TODO: assert_eq!(slice_segment_header.slice_segment_address, 192);
    // TODO: assert_eq!(slice_segment_header.slice_type, 3);
    // TODO: assert_eq!(slice_segment_header.slice_sao_luma_flag, 1);
    // TODO: assert_eq!(slice_segment_header.slice_sao_chroma_flag, 0);
    // TODO: assert_eq!(slice_segment_header.slice_qp_delta, 15);
}
