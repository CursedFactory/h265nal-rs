use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawPredWeightTableFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PredWeightTableFields {
    pub chroma_array_type: u32,
    pub num_ref_idx_l0_active_minus1: u32,
    pub luma_log2_weight_denom: u32,
    pub delta_chroma_log2_weight_denom: i32,
    pub luma_weight_l0_flag: Vec<u32>,
    pub chroma_weight_l0_flag: Vec<u32>,
}

pub fn pred_weight_table_parse(
    data: &[u8],
    chroma_array_type: u32,
    num_ref_idx_l0_active_minus1: u32,
) -> Result<PredWeightTableFields, Error> {
    let mut raw = RawPredWeightTableFields {
        chroma_array_type: 0,
        num_ref_idx_l0_active_minus1: 0,
        luma_log2_weight_denom: 0,
        delta_chroma_log2_weight_denom: 0,
        luma_weight_l0_flag_size: 0,
        luma_weight_l0_flag: [0; 16],
        chroma_weight_l0_flag_size: 0,
        chroma_weight_l0_flag: [0; 16],
    };
    let status = unsafe {
        ffi::h265nal_pred_weight_table_parse(
            data.as_ptr(),
            data.len(),
            chroma_array_type,
            num_ref_idx_l0_active_minus1,
            &mut raw,
        )
    };
    status_to_result(status)?;

    Ok(PredWeightTableFields {
        chroma_array_type: raw.chroma_array_type,
        num_ref_idx_l0_active_minus1: raw.num_ref_idx_l0_active_minus1,
        luma_log2_weight_denom: raw.luma_log2_weight_denom,
        delta_chroma_log2_weight_denom: raw.delta_chroma_log2_weight_denom,
        luma_weight_l0_flag: raw.luma_weight_l0_flag[..raw.luma_weight_l0_flag_size].to_vec(),
        chroma_weight_l0_flag: raw.chroma_weight_l0_flag[..raw.chroma_weight_l0_flag_size].to_vec(),
    })
}
