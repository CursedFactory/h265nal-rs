use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawSeiMessageFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeiMessageFields {
    pub payload_type: i32,
    pub payload_size: u32,
    pub has_user_data_registered_itu_t_t35: u32,
    pub user_data_registered_itu_t_t35_country_code: u8,
    pub user_data_registered_itu_t_t35_country_code_extension_byte: u8,
    pub has_user_data_unregistered: u32,
    pub user_data_unregistered_uuid_iso_iec_11578_1: u64,
    pub user_data_unregistered_uuid_iso_iec_11578_2: u64,
    pub has_alpha_channel_info: u32,
    pub alpha_channel_cancel_flag: u32,
    pub alpha_channel_use_idc: u32,
    pub alpha_channel_bit_depth_minus8: u32,
    pub alpha_transparent_value: u32,
    pub alpha_opaque_value: u32,
    pub alpha_channel_incr_flag: u32,
    pub alpha_channel_clip_flag: u32,
    pub alpha_channel_clip_type_flag: u32,
    pub has_mastering_display_colour_volume: u32,
    pub mastering_display_display_primaries_x: [u16; 3],
    pub mastering_display_display_primaries_y: [u16; 3],
    pub mastering_display_white_point_x: u16,
    pub mastering_display_white_point_y: u16,
    pub mastering_display_max_display_mastering_luminance: u32,
    pub mastering_display_min_display_mastering_luminance: u32,
    pub has_content_light_level_info: u32,
    pub content_light_level_max_content_light_level: u16,
    pub content_light_level_max_pic_average_light_level: u16,
}

/// Parses one SEI message and returns baseline parity fields.
///
/// Native references:
/// - Declaration: `include/h265_sei_parser.h:302` (`ParseSei`)
/// - Unit test: `test/h265_sei_parser_unittest.cc:50`
pub fn sei_parse(data: &[u8]) -> Result<SeiMessageFields, Error> {
    let mut raw = RawSeiMessageFields {
        payload_type: 0,
        payload_size: 0,
        has_user_data_registered_itu_t_t35: 0,
        user_data_registered_itu_t_t35_country_code: 0,
        user_data_registered_itu_t_t35_country_code_extension_byte: 0,
        has_user_data_unregistered: 0,
        user_data_unregistered_uuid_iso_iec_11578_1: 0,
        user_data_unregistered_uuid_iso_iec_11578_2: 0,
        has_alpha_channel_info: 0,
        alpha_channel_cancel_flag: 0,
        alpha_channel_use_idc: 0,
        alpha_channel_bit_depth_minus8: 0,
        alpha_transparent_value: 0,
        alpha_opaque_value: 0,
        alpha_channel_incr_flag: 0,
        alpha_channel_clip_flag: 0,
        alpha_channel_clip_type_flag: 0,
        has_mastering_display_colour_volume: 0,
        mastering_display_display_primaries_x: [0, 0, 0],
        mastering_display_display_primaries_y: [0, 0, 0],
        mastering_display_white_point_x: 0,
        mastering_display_white_point_y: 0,
        mastering_display_max_display_mastering_luminance: 0,
        mastering_display_min_display_mastering_luminance: 0,
        has_content_light_level_info: 0,
        content_light_level_max_content_light_level: 0,
        content_light_level_max_pic_average_light_level: 0,
    };
    let status = unsafe { ffi::h265nal_sei_parse(data.as_ptr(), data.len(), &mut raw) };
    status_to_result(status)?;

    Ok(SeiMessageFields {
        payload_type: raw.payload_type,
        payload_size: raw.payload_size,
        has_user_data_registered_itu_t_t35: raw.has_user_data_registered_itu_t_t35,
        user_data_registered_itu_t_t35_country_code: raw
            .user_data_registered_itu_t_t35_country_code,
        user_data_registered_itu_t_t35_country_code_extension_byte: raw
            .user_data_registered_itu_t_t35_country_code_extension_byte,
        has_user_data_unregistered: raw.has_user_data_unregistered,
        user_data_unregistered_uuid_iso_iec_11578_1: raw
            .user_data_unregistered_uuid_iso_iec_11578_1,
        user_data_unregistered_uuid_iso_iec_11578_2: raw
            .user_data_unregistered_uuid_iso_iec_11578_2,
        has_alpha_channel_info: raw.has_alpha_channel_info,
        alpha_channel_cancel_flag: raw.alpha_channel_cancel_flag,
        alpha_channel_use_idc: raw.alpha_channel_use_idc,
        alpha_channel_bit_depth_minus8: raw.alpha_channel_bit_depth_minus8,
        alpha_transparent_value: raw.alpha_transparent_value,
        alpha_opaque_value: raw.alpha_opaque_value,
        alpha_channel_incr_flag: raw.alpha_channel_incr_flag,
        alpha_channel_clip_flag: raw.alpha_channel_clip_flag,
        alpha_channel_clip_type_flag: raw.alpha_channel_clip_type_flag,
        has_mastering_display_colour_volume: raw.has_mastering_display_colour_volume,
        mastering_display_display_primaries_x: raw.mastering_display_display_primaries_x,
        mastering_display_display_primaries_y: raw.mastering_display_display_primaries_y,
        mastering_display_white_point_x: raw.mastering_display_white_point_x,
        mastering_display_white_point_y: raw.mastering_display_white_point_y,
        mastering_display_max_display_mastering_luminance: raw
            .mastering_display_max_display_mastering_luminance,
        mastering_display_min_display_mastering_luminance: raw
            .mastering_display_min_display_mastering_luminance,
        has_content_light_level_info: raw.has_content_light_level_info,
        content_light_level_max_content_light_level: raw
            .content_light_level_max_content_light_level,
        content_light_level_max_pic_average_light_level: raw
            .content_light_level_max_pic_average_light_level,
    })
}
