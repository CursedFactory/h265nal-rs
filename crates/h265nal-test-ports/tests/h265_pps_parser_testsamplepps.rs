//! References:
//! - Markdown: `docs/test-cases/h265_pps_parser_unittest/h265ppsparsertest--testsamplepps.md`
//! - C++: `test/h265_pps_parser_unittest.cc:21`
//! - Port note: Group 04 / Case 17

#[test]
fn test_sample_pps() {
    // PPS example
    let buffer = [0xc0u8, 0xf3, 0xc0, 0x02, 0x10, 0x00];
    let pps = h265nal_sys::pps_parse(&buffer).expect("ParsePps failed");

    assert_eq!(pps.pps_pic_parameter_set_id, 0);
    assert_eq!(pps.pps_seq_parameter_set_id, 0);
    assert_eq!(pps.dependent_slice_segments_enabled_flag, 0);
    assert_eq!(pps.output_flag_present_flag, 0);
    assert_eq!(pps.num_extra_slice_header_bits, 0);
    assert_eq!(pps.sign_data_hiding_enabled_flag, 0);
    assert_eq!(pps.cabac_init_present_flag, 1);
    assert_eq!(pps.num_ref_idx_l0_default_active_minus1, 0);
    assert_eq!(pps.num_ref_idx_l1_default_active_minus1, 0);
    assert_eq!(pps.init_qp_minus26, 0);
    assert_eq!(pps.constrained_intra_pred_flag, 0);
    assert_eq!(pps.transform_skip_enabled_flag, 0);
    assert_eq!(pps.cu_qp_delta_enabled_flag, 1);
    assert_eq!(pps.diff_cu_qp_delta_depth, 0);
    assert_eq!(pps.pps_cb_qp_offset, 0);
    assert_eq!(pps.pps_cr_qp_offset, 0);
    assert_eq!(pps.pps_slice_chroma_qp_offsets_present_flag, 0);
    assert_eq!(pps.weighted_pred_flag, 0);
    assert_eq!(pps.weighted_bipred_flag, 0);
    assert_eq!(pps.transquant_bypass_enabled_flag, 0);
    assert_eq!(pps.tiles_enabled_flag, 0);
    assert_eq!(pps.entropy_coding_sync_enabled_flag, 0);
    assert_eq!(pps.pps_loop_filter_across_slices_enabled_flag, 0);
    assert_eq!(pps.deblocking_filter_control_present_flag, 0);
    assert_eq!(pps.pps_scaling_list_data_present_flag, 0);
    assert_eq!(pps.lists_modification_present_flag, 0);
    assert_eq!(pps.log2_parallel_merge_level_minus2, 3);
    assert_eq!(pps.slice_segment_header_extension_present_flag, 0);
    assert_eq!(pps.pps_extension_present_flag, 0);
}
