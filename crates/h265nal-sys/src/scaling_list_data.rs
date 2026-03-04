use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawScalingListDataFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScalingListDataFields {
    pub size_id_count: u32,
    pub matrix_id_count: u32,
    pub scaling_list_pred_mode_flag: [u32; 24],
    pub scaling_list_pred_matrix_id_delta: [u32; 24],
    pub scaling_list_dc_coef_minus8: [i32; 24],
    pub scaling_list_size: [usize; 24],
    pub scaling_list_first_coef: [u32; 24],
    pub scaling_list_all_8: [u32; 24],
}

pub fn scaling_list_data_parse(data: &[u8]) -> Result<ScalingListDataFields, Error> {
    let mut raw = RawScalingListDataFields {
        size_id_count: 0,
        matrix_id_count: 0,
        scaling_list_pred_mode_flag: [0; 24],
        scaling_list_pred_matrix_id_delta: [0; 24],
        scaling_list_dc_coef_minus8: [0; 24],
        scaling_list_size: [0; 24],
        scaling_list_first_coef: [0; 24],
        scaling_list_all_8: [0; 24],
    };
    let status =
        unsafe { ffi::h265nal_scaling_list_data_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;

    Ok(ScalingListDataFields {
        size_id_count: raw.size_id_count,
        matrix_id_count: raw.matrix_id_count,
        scaling_list_pred_mode_flag: raw.scaling_list_pred_mode_flag,
        scaling_list_pred_matrix_id_delta: raw.scaling_list_pred_matrix_id_delta,
        scaling_list_dc_coef_minus8: raw.scaling_list_dc_coef_minus8,
        scaling_list_size: raw.scaling_list_size,
        scaling_list_first_coef: raw.scaling_list_first_coef,
        scaling_list_all_8: raw.scaling_list_all_8,
    })
}
