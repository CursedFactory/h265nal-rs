use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawRtpFields};
use crate::state::BitstreamParserState;

pub const RTP_PACKET_KIND_NONE: u32 = 0;
pub const RTP_PACKET_KIND_SINGLE: u32 = 1;
pub const RTP_PACKET_KIND_AP: u32 = 2;
pub const RTP_PACKET_KIND_FU: u32 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RtpFields {
    pub forbidden_zero_bit: u32,
    pub nal_unit_type: u32,
    pub nuh_layer_id: u32,
    pub nuh_temporal_id_plus1: u32,
    pub packet_kind: u32,
    pub has_payload_sps: u32,
    pub payload_sps_seq_parameter_set_id: u32,
    pub payload_sps_pic_width_in_luma_samples: u32,
    pub payload_sps_pic_height_in_luma_samples: u32,
    pub has_payload_pps: u32,
    pub payload_pps_pic_parameter_set_id: u32,
    pub payload_pps_init_qp_minus26: i32,
    pub has_payload_slice_segment_header: u32,
    pub payload_slice_nal_unit_type: u32,
    pub payload_slice_first_slice_segment_in_pic_flag: u32,
    pub payload_slice_no_output_of_prior_pics_flag: u32,
    pub payload_slice_pic_parameter_set_id: u32,
    pub payload_slice_type: u32,
    pub payload_slice_qp_delta: i32,
    pub ap_nal_unit_sizes_count: usize,
    pub ap_nal_unit_headers_count: usize,
    pub ap_nal_unit_payloads_count: usize,
    pub ap_nal_unit_types: Vec<u32>,
    pub fu_s_bit: u32,
    pub fu_e_bit: u32,
    pub fu_type: u32,
    pub fu_has_nal_unit_payload: u32,
}

/// Parses an RTP packet and exposes minimal flattened fields used by RTP parity tests.
///
/// Native references:
/// - Declaration: `include/h265_rtp_parser.h:45` (`ParseRtp`)
/// - Unit tests: `test/h265_rtp_parser_unittest.cc:22`
/// - Unit tests: `test/h265_rtp_parser_unittest.cc:59`
pub fn rtp_parse(data: &[u8], state: &mut BitstreamParserState) -> Result<RtpFields, Error> {
    let mut raw: RawRtpFields = unsafe { std::mem::zeroed() };
    let status =
        unsafe { ffi::h265nal_rtp_parse(data.as_ptr(), data.len(), state.as_mut_ptr(), &mut raw) };
    status_to_result(status)?;

    Ok(RtpFields {
        forbidden_zero_bit: raw.forbidden_zero_bit,
        nal_unit_type: raw.nal_unit_type,
        nuh_layer_id: raw.nuh_layer_id,
        nuh_temporal_id_plus1: raw.nuh_temporal_id_plus1,
        packet_kind: raw.packet_kind,
        has_payload_sps: raw.has_payload_sps,
        payload_sps_seq_parameter_set_id: raw.payload_sps_seq_parameter_set_id,
        payload_sps_pic_width_in_luma_samples: raw.payload_sps_pic_width_in_luma_samples,
        payload_sps_pic_height_in_luma_samples: raw.payload_sps_pic_height_in_luma_samples,
        has_payload_pps: raw.has_payload_pps,
        payload_pps_pic_parameter_set_id: raw.payload_pps_pic_parameter_set_id,
        payload_pps_init_qp_minus26: raw.payload_pps_init_qp_minus26,
        has_payload_slice_segment_header: raw.has_payload_slice_segment_header,
        payload_slice_nal_unit_type: raw.payload_slice_nal_unit_type,
        payload_slice_first_slice_segment_in_pic_flag: raw
            .payload_slice_first_slice_segment_in_pic_flag,
        payload_slice_no_output_of_prior_pics_flag: raw.payload_slice_no_output_of_prior_pics_flag,
        payload_slice_pic_parameter_set_id: raw.payload_slice_pic_parameter_set_id,
        payload_slice_type: raw.payload_slice_type,
        payload_slice_qp_delta: raw.payload_slice_qp_delta,
        ap_nal_unit_sizes_count: raw.ap_nal_unit_sizes_count,
        ap_nal_unit_headers_count: raw.ap_nal_unit_headers_count,
        ap_nal_unit_payloads_count: raw.ap_nal_unit_payloads_count,
        ap_nal_unit_types: raw.ap_nal_unit_types[..raw.ap_nal_unit_type_count].to_vec(),
        fu_s_bit: raw.fu_s_bit,
        fu_e_bit: raw.fu_e_bit,
        fu_type: raw.fu_type,
        fu_has_nal_unit_payload: raw.fu_has_nal_unit_payload,
    })
}
