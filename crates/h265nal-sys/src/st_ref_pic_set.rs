use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawStRefPicSetFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StRefPicSetFields {
    pub st_rps_idx: u32,
    pub num_short_term_ref_pic_sets: u32,
    pub max_num_pics: u32,
    pub num_negative_pics: u32,
    pub num_positive_pics: u32,
    pub delta_poc_s0_minus1_size: usize,
    pub delta_poc_s0_minus1: [u32; 16],
}

pub fn st_ref_pic_set_parse(
    data: &[u8],
    st_rps_idx: u32,
    num_short_term_ref_pic_sets: u32,
    max_num_pics: u32,
) -> Result<StRefPicSetFields, Error> {
    let mut raw = RawStRefPicSetFields {
        st_rps_idx: 0,
        num_short_term_ref_pic_sets: 0,
        max_num_pics: 0,
        num_negative_pics: 0,
        num_positive_pics: 0,
        delta_poc_s0_minus1_size: 0,
        delta_poc_s0_minus1: [0; 16],
    };
    let status = unsafe {
        ffi::h265nal_st_ref_pic_set_parse(
            data.as_ptr(),
            data.len(),
            st_rps_idx,
            num_short_term_ref_pic_sets,
            max_num_pics,
            &mut raw,
        )
    };
    status_to_result(status)?;

    Ok(StRefPicSetFields {
        st_rps_idx: raw.st_rps_idx,
        num_short_term_ref_pic_sets: raw.num_short_term_ref_pic_sets,
        max_num_pics: raw.max_num_pics,
        num_negative_pics: raw.num_negative_pics,
        num_positive_pics: raw.num_positive_pics,
        delta_poc_s0_minus1_size: raw.delta_poc_s0_minus1_size,
        delta_poc_s0_minus1: raw.delta_poc_s0_minus1,
    })
}
