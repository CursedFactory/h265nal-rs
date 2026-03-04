use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawPpsFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PpsFields {
    pub pps_pic_parameter_set_id: u32,
    pub pps_seq_parameter_set_id: u32,
    pub dependent_slice_segments_enabled_flag: u32,
    pub output_flag_present_flag: u32,
    pub num_extra_slice_header_bits: u32,
    pub sign_data_hiding_enabled_flag: u32,
    pub cabac_init_present_flag: u32,
    pub num_ref_idx_l0_default_active_minus1: u32,
    pub num_ref_idx_l1_default_active_minus1: u32,
    pub init_qp_minus26: i32,
    pub constrained_intra_pred_flag: u32,
    pub transform_skip_enabled_flag: u32,
    pub cu_qp_delta_enabled_flag: u32,
    pub diff_cu_qp_delta_depth: u32,
    pub pps_cb_qp_offset: i32,
    pub pps_cr_qp_offset: i32,
    pub pps_slice_chroma_qp_offsets_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub weighted_bipred_flag: u32,
    pub transquant_bypass_enabled_flag: u32,
    pub tiles_enabled_flag: u32,
    pub entropy_coding_sync_enabled_flag: u32,
    pub pps_loop_filter_across_slices_enabled_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub pps_scaling_list_data_present_flag: u32,
    pub lists_modification_present_flag: u32,
    pub log2_parallel_merge_level_minus2: u32,
    pub slice_segment_header_extension_present_flag: u32,
    pub pps_extension_present_flag: u32,
}

/// Parses PPS and returns the core fields checked by native tests.
///
/// Native references:
/// - Declaration: `include/h265_pps_parser.h:96` (`ParsePps`)
/// - Unit test: `test/h265_pps_parser_unittest.cc:21`
pub fn pps_parse(data: &[u8]) -> Result<PpsFields, Error> {
    let mut raw = RawPpsFields {
        pps_pic_parameter_set_id: 0,
        pps_seq_parameter_set_id: 0,
        dependent_slice_segments_enabled_flag: 0,
        output_flag_present_flag: 0,
        num_extra_slice_header_bits: 0,
        sign_data_hiding_enabled_flag: 0,
        cabac_init_present_flag: 0,
        num_ref_idx_l0_default_active_minus1: 0,
        num_ref_idx_l1_default_active_minus1: 0,
        init_qp_minus26: 0,
        constrained_intra_pred_flag: 0,
        transform_skip_enabled_flag: 0,
        cu_qp_delta_enabled_flag: 0,
        diff_cu_qp_delta_depth: 0,
        pps_cb_qp_offset: 0,
        pps_cr_qp_offset: 0,
        pps_slice_chroma_qp_offsets_present_flag: 0,
        weighted_pred_flag: 0,
        weighted_bipred_flag: 0,
        transquant_bypass_enabled_flag: 0,
        tiles_enabled_flag: 0,
        entropy_coding_sync_enabled_flag: 0,
        pps_loop_filter_across_slices_enabled_flag: 0,
        deblocking_filter_control_present_flag: 0,
        pps_scaling_list_data_present_flag: 0,
        lists_modification_present_flag: 0,
        log2_parallel_merge_level_minus2: 0,
        slice_segment_header_extension_present_flag: 0,
        pps_extension_present_flag: 0,
    };
    let status = unsafe { ffi::h265nal_pps_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;

    Ok(PpsFields {
        pps_pic_parameter_set_id: raw.pps_pic_parameter_set_id,
        pps_seq_parameter_set_id: raw.pps_seq_parameter_set_id,
        dependent_slice_segments_enabled_flag: raw.dependent_slice_segments_enabled_flag,
        output_flag_present_flag: raw.output_flag_present_flag,
        num_extra_slice_header_bits: raw.num_extra_slice_header_bits,
        sign_data_hiding_enabled_flag: raw.sign_data_hiding_enabled_flag,
        cabac_init_present_flag: raw.cabac_init_present_flag,
        num_ref_idx_l0_default_active_minus1: raw.num_ref_idx_l0_default_active_minus1,
        num_ref_idx_l1_default_active_minus1: raw.num_ref_idx_l1_default_active_minus1,
        init_qp_minus26: raw.init_qp_minus26,
        constrained_intra_pred_flag: raw.constrained_intra_pred_flag,
        transform_skip_enabled_flag: raw.transform_skip_enabled_flag,
        cu_qp_delta_enabled_flag: raw.cu_qp_delta_enabled_flag,
        diff_cu_qp_delta_depth: raw.diff_cu_qp_delta_depth,
        pps_cb_qp_offset: raw.pps_cb_qp_offset,
        pps_cr_qp_offset: raw.pps_cr_qp_offset,
        pps_slice_chroma_qp_offsets_present_flag: raw.pps_slice_chroma_qp_offsets_present_flag,
        weighted_pred_flag: raw.weighted_pred_flag,
        weighted_bipred_flag: raw.weighted_bipred_flag,
        transquant_bypass_enabled_flag: raw.transquant_bypass_enabled_flag,
        tiles_enabled_flag: raw.tiles_enabled_flag,
        entropy_coding_sync_enabled_flag: raw.entropy_coding_sync_enabled_flag,
        pps_loop_filter_across_slices_enabled_flag: raw.pps_loop_filter_across_slices_enabled_flag,
        deblocking_filter_control_present_flag: raw.deblocking_filter_control_present_flag,
        pps_scaling_list_data_present_flag: raw.pps_scaling_list_data_present_flag,
        lists_modification_present_flag: raw.lists_modification_present_flag,
        log2_parallel_merge_level_minus2: raw.log2_parallel_merge_level_minus2,
        slice_segment_header_extension_present_flag: raw
            .slice_segment_header_extension_present_flag,
        pps_extension_present_flag: raw.pps_extension_present_flag,
    })
}
