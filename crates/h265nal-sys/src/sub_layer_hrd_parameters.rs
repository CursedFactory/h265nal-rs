use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawSubLayerHrdParametersFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubLayerHrdParametersFields {
    pub sub_layer_id: u32,
    pub cpb_cnt: u32,
    pub sub_pic_hrd_params_present_flag: u32,
    pub bit_rate_value_minus1_size: usize,
    pub bit_rate_value_minus1: [u32; 32],
    pub cpb_size_value_minus1_size: usize,
    pub cpb_size_value_minus1: [u32; 32],
    pub cpb_size_du_value_minus1_size: usize,
    pub cpb_size_du_value_minus1: [u32; 32],
    pub bit_rate_du_value_minus1_size: usize,
    pub bit_rate_du_value_minus1: [u32; 32],
    pub cbr_flag_size: usize,
    pub cbr_flag: [u32; 32],
}

pub fn sub_layer_hrd_parameters_parse(
    data: &[u8],
    sub_layer_id: u32,
    cpb_cnt: u32,
    sub_pic_hrd_params_present_flag: bool,
) -> Result<SubLayerHrdParametersFields, Error> {
    let mut raw = RawSubLayerHrdParametersFields {
        sub_layer_id: 0,
        cpb_cnt: 0,
        sub_pic_hrd_params_present_flag: 0,
        bit_rate_value_minus1_size: 0,
        bit_rate_value_minus1: [0; 32],
        cpb_size_value_minus1_size: 0,
        cpb_size_value_minus1: [0; 32],
        cpb_size_du_value_minus1_size: 0,
        cpb_size_du_value_minus1: [0; 32],
        bit_rate_du_value_minus1_size: 0,
        bit_rate_du_value_minus1: [0; 32],
        cbr_flag_size: 0,
        cbr_flag: [0; 32],
    };
    let status = unsafe {
        ffi::h265nal_sub_layer_hrd_parameters_parse(
            data.as_ptr(),
            data.len(),
            sub_layer_id,
            cpb_cnt,
            u32::from(sub_pic_hrd_params_present_flag),
            &mut raw,
        )
    };
    status_to_result(status)?;

    Ok(SubLayerHrdParametersFields {
        sub_layer_id: raw.sub_layer_id,
        cpb_cnt: raw.cpb_cnt,
        sub_pic_hrd_params_present_flag: raw.sub_pic_hrd_params_present_flag,
        bit_rate_value_minus1_size: raw.bit_rate_value_minus1_size,
        bit_rate_value_minus1: raw.bit_rate_value_minus1,
        cpb_size_value_minus1_size: raw.cpb_size_value_minus1_size,
        cpb_size_value_minus1: raw.cpb_size_value_minus1,
        cpb_size_du_value_minus1_size: raw.cpb_size_du_value_minus1_size,
        cpb_size_du_value_minus1: raw.cpb_size_du_value_minus1,
        bit_rate_du_value_minus1_size: raw.bit_rate_du_value_minus1_size,
        bit_rate_du_value_minus1: raw.bit_rate_du_value_minus1,
        cbr_flag_size: raw.cbr_flag_size,
        cbr_flag: raw.cbr_flag,
    })
}
