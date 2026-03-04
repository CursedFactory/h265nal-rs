use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawHrdParametersFields};
use crate::sub_layer_hrd_parameters::SubLayerHrdParametersFields;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HrdParametersFields {
    pub common_inf_present_flag: u32,
    pub max_num_sub_layers_minus1: u32,
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
    pub sub_pic_hrd_params_present_flag: u32,
    pub tick_divisor_minus2: u32,
    pub du_cpb_removal_delay_increment_length_minus1: u32,
    pub sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
    pub dpb_output_delay_du_length_minus1: u32,
    pub bit_rate_scale: u32,
    pub cpb_size_scale: u32,
    pub cpb_size_du_scale: u32,
    pub initial_cpb_removal_delay_length_minus1: u32,
    pub au_cpb_removal_delay_length_minus1: u32,
    pub dpb_output_delay_length_minus1: u32,
    pub fixed_pic_rate_general_flag_size: usize,
    pub fixed_pic_rate_general_flag: [u32; 8],
    pub fixed_pic_rate_within_cvs_flag_size: usize,
    pub fixed_pic_rate_within_cvs_flag: [u32; 8],
    pub elemental_duration_in_tc_minus1_size: usize,
    pub elemental_duration_in_tc_minus1: [u32; 8],
    pub low_delay_hrd_flag_size: usize,
    pub low_delay_hrd_flag: [u32; 8],
    pub cpb_cnt_minus1_size: usize,
    pub cpb_cnt_minus1: [u32; 8],
    pub sub_layer_hrd_parameters_vector_size: usize,
    pub sub_layer_hrd_parameters_0_present: u32,
    pub sub_layer_hrd_parameters_0: SubLayerHrdParametersFields,
}

/// Parses HRD parameters with explicit parser options from the caller.
///
/// Native references:
/// - Declaration: `include/h265_hrd_parameters_parser.h:64` (`ParseHrdParameters`)
/// - Unit test: `test/h265_hrd_parameters_parser_unittest.cc:21`
pub fn hrd_parameters_parse(
    data: &[u8],
    common_inf_present_flag: u32,
    max_num_sub_layers_minus1: u32,
) -> Result<HrdParametersFields, Error> {
    let mut raw = RawHrdParametersFields {
        common_inf_present_flag: 0,
        max_num_sub_layers_minus1: 0,
        nal_hrd_parameters_present_flag: 0,
        vcl_hrd_parameters_present_flag: 0,
        sub_pic_hrd_params_present_flag: 0,
        tick_divisor_minus2: 0,
        du_cpb_removal_delay_increment_length_minus1: 0,
        sub_pic_cpb_params_in_pic_timing_sei_flag: 0,
        dpb_output_delay_du_length_minus1: 0,
        bit_rate_scale: 0,
        cpb_size_scale: 0,
        cpb_size_du_scale: 0,
        initial_cpb_removal_delay_length_minus1: 0,
        au_cpb_removal_delay_length_minus1: 0,
        dpb_output_delay_length_minus1: 0,
        fixed_pic_rate_general_flag_size: 0,
        fixed_pic_rate_general_flag: [0; 8],
        fixed_pic_rate_within_cvs_flag_size: 0,
        fixed_pic_rate_within_cvs_flag: [0; 8],
        elemental_duration_in_tc_minus1_size: 0,
        elemental_duration_in_tc_minus1: [0; 8],
        low_delay_hrd_flag_size: 0,
        low_delay_hrd_flag: [0; 8],
        cpb_cnt_minus1_size: 0,
        cpb_cnt_minus1: [0; 8],
        sub_layer_hrd_parameters_vector_size: 0,
        sub_layer_hrd_parameters_0_present: 0,
        sub_layer_hrd_parameters_0: crate::ffi::RawSubLayerHrdParametersFields {
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
        },
    };
    let status = unsafe {
        ffi::h265nal_hrd_parameters_parse(
            data.as_ptr(),
            data.len(),
            common_inf_present_flag,
            max_num_sub_layers_minus1,
            &mut raw,
        )
    };
    status_to_result(status)?;

    Ok(HrdParametersFields {
        common_inf_present_flag: raw.common_inf_present_flag,
        max_num_sub_layers_minus1: raw.max_num_sub_layers_minus1,
        nal_hrd_parameters_present_flag: raw.nal_hrd_parameters_present_flag,
        vcl_hrd_parameters_present_flag: raw.vcl_hrd_parameters_present_flag,
        sub_pic_hrd_params_present_flag: raw.sub_pic_hrd_params_present_flag,
        tick_divisor_minus2: raw.tick_divisor_minus2,
        du_cpb_removal_delay_increment_length_minus1: raw
            .du_cpb_removal_delay_increment_length_minus1,
        sub_pic_cpb_params_in_pic_timing_sei_flag: raw.sub_pic_cpb_params_in_pic_timing_sei_flag,
        dpb_output_delay_du_length_minus1: raw.dpb_output_delay_du_length_minus1,
        bit_rate_scale: raw.bit_rate_scale,
        cpb_size_scale: raw.cpb_size_scale,
        cpb_size_du_scale: raw.cpb_size_du_scale,
        initial_cpb_removal_delay_length_minus1: raw.initial_cpb_removal_delay_length_minus1,
        au_cpb_removal_delay_length_minus1: raw.au_cpb_removal_delay_length_minus1,
        dpb_output_delay_length_minus1: raw.dpb_output_delay_length_minus1,
        fixed_pic_rate_general_flag_size: raw.fixed_pic_rate_general_flag_size,
        fixed_pic_rate_general_flag: raw.fixed_pic_rate_general_flag,
        fixed_pic_rate_within_cvs_flag_size: raw.fixed_pic_rate_within_cvs_flag_size,
        fixed_pic_rate_within_cvs_flag: raw.fixed_pic_rate_within_cvs_flag,
        elemental_duration_in_tc_minus1_size: raw.elemental_duration_in_tc_minus1_size,
        elemental_duration_in_tc_minus1: raw.elemental_duration_in_tc_minus1,
        low_delay_hrd_flag_size: raw.low_delay_hrd_flag_size,
        low_delay_hrd_flag: raw.low_delay_hrd_flag,
        cpb_cnt_minus1_size: raw.cpb_cnt_minus1_size,
        cpb_cnt_minus1: raw.cpb_cnt_minus1,
        sub_layer_hrd_parameters_vector_size: raw.sub_layer_hrd_parameters_vector_size,
        sub_layer_hrd_parameters_0_present: raw.sub_layer_hrd_parameters_0_present,
        sub_layer_hrd_parameters_0: SubLayerHrdParametersFields {
            sub_layer_id: raw.sub_layer_hrd_parameters_0.sub_layer_id,
            cpb_cnt: raw.sub_layer_hrd_parameters_0.cpb_cnt,
            sub_pic_hrd_params_present_flag: raw
                .sub_layer_hrd_parameters_0
                .sub_pic_hrd_params_present_flag,
            bit_rate_value_minus1_size: raw.sub_layer_hrd_parameters_0.bit_rate_value_minus1_size,
            bit_rate_value_minus1: raw.sub_layer_hrd_parameters_0.bit_rate_value_minus1,
            cpb_size_value_minus1_size: raw.sub_layer_hrd_parameters_0.cpb_size_value_minus1_size,
            cpb_size_value_minus1: raw.sub_layer_hrd_parameters_0.cpb_size_value_minus1,
            cpb_size_du_value_minus1_size: raw
                .sub_layer_hrd_parameters_0
                .cpb_size_du_value_minus1_size,
            cpb_size_du_value_minus1: raw.sub_layer_hrd_parameters_0.cpb_size_du_value_minus1,
            bit_rate_du_value_minus1_size: raw
                .sub_layer_hrd_parameters_0
                .bit_rate_du_value_minus1_size,
            bit_rate_du_value_minus1: raw.sub_layer_hrd_parameters_0.bit_rate_du_value_minus1,
            cbr_flag_size: raw.sub_layer_hrd_parameters_0.cbr_flag_size,
            cbr_flag: raw.sub_layer_hrd_parameters_0.cbr_flag,
        },
    })
}
