use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawProfileInfoFields, RawProfileTierLevelFields};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileInfoFields {
    pub profile_space: u32,
    pub tier_flag: u32,
    pub profile_idc: u32,
    pub profile_compatibility_flag: [u32; 32],
    pub progressive_source_flag: u32,
    pub interlaced_source_flag: u32,
    pub non_packed_constraint_flag: u32,
    pub frame_only_constraint_flag: u32,
    pub max_12bit_constraint_flag: u32,
    pub max_10bit_constraint_flag: u32,
    pub max_8bit_constraint_flag: u32,
    pub max_422chroma_constraint_flag: u32,
    pub max_420chroma_constraint_flag: u32,
    pub max_monochrome_constraint_flag: u32,
    pub intra_constraint_flag: u32,
    pub one_picture_only_constraint_flag: u32,
    pub lower_bit_rate_constraint_flag: u32,
    pub max_14bit_constraint_flag: u32,
    pub reserved_zero_33bits: u64,
    pub reserved_zero_34bits: u64,
    pub reserved_zero_43bits: u64,
    pub inbld_flag: u32,
    pub reserved_zero_bit: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileTierLevelFields {
    pub general: ProfileInfoFields,
    pub general_level_idc: u32,
    pub sub_layer_profile_present_flag_size: usize,
    pub sub_layer_level_present_flag_size: usize,
    pub reserved_zero_2bits_size: usize,
    pub sub_layer_size: usize,
}

/// Parses profile_tier_level and returns the fields asserted by native tests.
///
/// Native references:
/// - Declaration: `include/h265_profile_tier_level_parser.h:112` (`ParseProfileTierLevel`)
/// - Unit tests: `test/h265_profile_tier_level_parser_unittest.cc:20`
/// - Unit tests: `test/h265_profile_tier_level_parser_unittest.cc:69`
pub fn profile_tier_level_parse(
    data: &[u8],
    profile_present_flag: bool,
    max_num_sub_layers_minus1: u32,
) -> Result<ProfileTierLevelFields, Error> {
    let mut raw = RawProfileTierLevelFields {
        general: empty_raw_profile_info(),
        general_level_idc: 0,
        sub_layer_profile_present_flag_size: 0,
        sub_layer_level_present_flag_size: 0,
        reserved_zero_2bits_size: 0,
        sub_layer_size: 0,
    };
    let status = unsafe {
        ffi::h265nal_profile_tier_level_parse(
            data.as_ptr(),
            data.len(),
            u32::from(profile_present_flag),
            max_num_sub_layers_minus1,
            &mut raw,
        )
    };
    status_to_result(status)?;

    Ok(ProfileTierLevelFields {
        general: ProfileInfoFields {
            profile_space: raw.general.profile_space,
            tier_flag: raw.general.tier_flag,
            profile_idc: raw.general.profile_idc,
            profile_compatibility_flag: raw.general.profile_compatibility_flag,
            progressive_source_flag: raw.general.progressive_source_flag,
            interlaced_source_flag: raw.general.interlaced_source_flag,
            non_packed_constraint_flag: raw.general.non_packed_constraint_flag,
            frame_only_constraint_flag: raw.general.frame_only_constraint_flag,
            max_12bit_constraint_flag: raw.general.max_12bit_constraint_flag,
            max_10bit_constraint_flag: raw.general.max_10bit_constraint_flag,
            max_8bit_constraint_flag: raw.general.max_8bit_constraint_flag,
            max_422chroma_constraint_flag: raw.general.max_422chroma_constraint_flag,
            max_420chroma_constraint_flag: raw.general.max_420chroma_constraint_flag,
            max_monochrome_constraint_flag: raw.general.max_monochrome_constraint_flag,
            intra_constraint_flag: raw.general.intra_constraint_flag,
            one_picture_only_constraint_flag: raw.general.one_picture_only_constraint_flag,
            lower_bit_rate_constraint_flag: raw.general.lower_bit_rate_constraint_flag,
            max_14bit_constraint_flag: raw.general.max_14bit_constraint_flag,
            reserved_zero_33bits: raw.general.reserved_zero_33bits,
            reserved_zero_34bits: raw.general.reserved_zero_34bits,
            reserved_zero_43bits: raw.general.reserved_zero_43bits,
            inbld_flag: raw.general.inbld_flag,
            reserved_zero_bit: raw.general.reserved_zero_bit,
        },
        general_level_idc: raw.general_level_idc,
        sub_layer_profile_present_flag_size: raw.sub_layer_profile_present_flag_size,
        sub_layer_level_present_flag_size: raw.sub_layer_level_present_flag_size,
        reserved_zero_2bits_size: raw.reserved_zero_2bits_size,
        sub_layer_size: raw.sub_layer_size,
    })
}

fn empty_raw_profile_info() -> RawProfileInfoFields {
    RawProfileInfoFields {
        profile_space: 0,
        tier_flag: 0,
        profile_idc: 0,
        profile_compatibility_flag: [0; 32],
        progressive_source_flag: 0,
        interlaced_source_flag: 0,
        non_packed_constraint_flag: 0,
        frame_only_constraint_flag: 0,
        max_12bit_constraint_flag: 0,
        max_10bit_constraint_flag: 0,
        max_8bit_constraint_flag: 0,
        max_422chroma_constraint_flag: 0,
        max_420chroma_constraint_flag: 0,
        max_monochrome_constraint_flag: 0,
        intra_constraint_flag: 0,
        one_picture_only_constraint_flag: 0,
        lower_bit_rate_constraint_flag: 0,
        max_14bit_constraint_flag: 0,
        reserved_zero_33bits: 0,
        reserved_zero_34bits: 0,
        reserved_zero_43bits: 0,
        inbld_flag: 0,
        reserved_zero_bit: 0,
    }
}
