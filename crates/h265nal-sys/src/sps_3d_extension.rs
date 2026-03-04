use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawSps3dExtensionFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sps3dExtensionFields {
    pub iv_di_mc_enabled_flag_size: usize,
    pub iv_di_mc_enabled_flag: [u32; 8],
    pub iv_mv_scal_enabled_flag_size: usize,
    pub iv_mv_scal_enabled_flag: [u32; 8],
    pub log2_ivmc_sub_pb_size_minus3: u32,
    pub iv_res_pred_enabled_flag: u32,
    pub depth_ref_enabled_flag: u32,
    pub vsp_mc_enabled_flag: u32,
    pub dbbp_enabled_flag: u32,
    pub tex_mc_enabled_flag: u32,
    pub log2_texmc_sub_pb_size_minus3: u32,
    pub intra_contour_enabled_flag: u32,
    pub intra_dc_only_wedge_enabled_flag: u32,
    pub cqt_cu_part_pred_enabled_flag: u32,
    pub inter_dc_only_enabled_flag: u32,
    pub skip_intra_enabled_flag: u32,
}

pub fn sps_3d_extension_parse(data: &[u8]) -> Result<Sps3dExtensionFields, Error> {
    let mut raw = RawSps3dExtensionFields {
        iv_di_mc_enabled_flag_size: 0,
        iv_di_mc_enabled_flag: [0; 8],
        iv_mv_scal_enabled_flag_size: 0,
        iv_mv_scal_enabled_flag: [0; 8],
        log2_ivmc_sub_pb_size_minus3: 0,
        iv_res_pred_enabled_flag: 0,
        depth_ref_enabled_flag: 0,
        vsp_mc_enabled_flag: 0,
        dbbp_enabled_flag: 0,
        tex_mc_enabled_flag: 0,
        log2_texmc_sub_pb_size_minus3: 0,
        intra_contour_enabled_flag: 0,
        intra_dc_only_wedge_enabled_flag: 0,
        cqt_cu_part_pred_enabled_flag: 0,
        inter_dc_only_enabled_flag: 0,
        skip_intra_enabled_flag: 0,
    };
    let status =
        unsafe { ffi::h265nal_sps_3d_extension_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;

    Ok(Sps3dExtensionFields {
        iv_di_mc_enabled_flag_size: raw.iv_di_mc_enabled_flag_size,
        iv_di_mc_enabled_flag: raw.iv_di_mc_enabled_flag,
        iv_mv_scal_enabled_flag_size: raw.iv_mv_scal_enabled_flag_size,
        iv_mv_scal_enabled_flag: raw.iv_mv_scal_enabled_flag,
        log2_ivmc_sub_pb_size_minus3: raw.log2_ivmc_sub_pb_size_minus3,
        iv_res_pred_enabled_flag: raw.iv_res_pred_enabled_flag,
        depth_ref_enabled_flag: raw.depth_ref_enabled_flag,
        vsp_mc_enabled_flag: raw.vsp_mc_enabled_flag,
        dbbp_enabled_flag: raw.dbbp_enabled_flag,
        tex_mc_enabled_flag: raw.tex_mc_enabled_flag,
        log2_texmc_sub_pb_size_minus3: raw.log2_texmc_sub_pb_size_minus3,
        intra_contour_enabled_flag: raw.intra_contour_enabled_flag,
        intra_dc_only_wedge_enabled_flag: raw.intra_dc_only_wedge_enabled_flag,
        cqt_cu_part_pred_enabled_flag: raw.cqt_cu_part_pred_enabled_flag,
        inter_dc_only_enabled_flag: raw.inter_dc_only_enabled_flag,
        skip_intra_enabled_flag: raw.skip_intra_enabled_flag,
    })
}
