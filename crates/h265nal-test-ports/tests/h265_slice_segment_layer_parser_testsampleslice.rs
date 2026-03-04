//! References:
//! - Markdown: `docs/test-cases/h265_slice_parser_unittest/h265slicesegmentlayerparsertest--testsampleslice.md`
//! - C++: `test/h265_slice_parser_unittest.cc:26`
//! - Port note: Group 08 / Case 38

#[test]
#[ignore = "TODO: missing slice_segment_layer_parse API"]
fn test_sample_slice() {
    // fuzzer::conv: data
    let buffer = [
        0xaf, 0x09, 0x40, 0xf3, 0xb8, 0xd5, 0x39, 0xba, 0x1f, 0xe4, 0xa6, 0x08, 0x5c, 0x6e, 0xb1,
        0x8f, 0x00, 0x38, 0xf1, 0xa6, 0xfc, 0xf1, 0x40, 0x04, 0x3a, 0x86, 0xcb, 0x90, 0x74, 0xce,
        0xf0, 0x46, 0x61, 0x93, 0x72, 0xd6, 0xfc, 0x35, 0xe3, 0xc5,
    ];

    // fuzzer::conv: begin
    // get some mock state
    let mut bitstream_parser_state =
        h265nal_sys::BitstreamParserState::new().expect("BitstreamParserState creation failed");
    // TODO: set up vps, sps, pps in state
    // auto vps = std::make_shared<H265VpsParser::VpsState>();
    // bitstream_parser_state.vps[0] = vps;
    // auto sps = std::make_shared<H265SpsParser::SpsState>();
    // sps->sample_adaptive_offset_enabled_flag = 1;
    // sps->chroma_format_idc = 1;
    // bitstream_parser_state.sps[0] = sps;
    // auto pps = std::make_shared<H265PpsParser::PpsState>();
    // bitstream_parser_state.pps[0] = pps;

    // TODO: auto slice_segment_layer = H265SliceSegmentLayerParser::ParseSliceSegmentLayer(buffer, arraysize(buffer), NalUnitType::IDR_W_RADL, &bitstream_parser_state);
    // TODO: EXPECT_TRUE(slice_segment_layer != nullptr);

    // TODO: auto& slice_segment_header = slice_segment_layer->slice_segment_header;

    // TODO: EXPECT_EQ(1, slice_segment_header->first_slice_segment_in_pic_flag);
    // TODO: EXPECT_EQ(0, slice_segment_header->no_output_of_prior_pics_flag);
    // TODO: EXPECT_EQ(0, slice_segment_header->slice_pic_parameter_set_id);
    // TODO: EXPECT_EQ(2, slice_segment_header->slice_type);
    // TODO: EXPECT_EQ(1, slice_segment_header->slice_sao_luma_flag);
    // TODO: EXPECT_EQ(1, slice_segment_header->slice_sao_chroma_flag);
    // TODO: EXPECT_EQ(9, slice_segment_header->slice_qp_delta);

    // fuzzer::conv: end
}
