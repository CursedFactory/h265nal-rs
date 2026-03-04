use crate::error::{status_to_result, Error};
use crate::ffi::{self, NalUnitTypePredicate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoreRbspDataResult {
    pub has_more_data: bool,
    pub byte_offset: usize,
    pub bit_offset: usize,
}

/// Returns true when `nal_unit_type` is a slice segment type.
///
/// Native references:
/// - Declaration: `include/h265_common.h:186` (`IsSliceSegment`)
/// - Unit test: `test/h265_common_unittest.cc:22`
pub fn common_is_slice_segment(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(nal_unit_type, ffi::h265nal_common_is_slice_segment)
}

/// Returns true when `nal_unit_type` is a VCL NAL unit type.
///
/// Native references:
/// - Declaration: `include/h265_common.h:188` (`IsNalUnitTypeVcl`)
/// - Unit test: `test/h265_common_unittest.cc:27`
pub fn common_is_nal_unit_type_vcl(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(nal_unit_type, ffi::h265nal_common_is_nal_unit_type_vcl)
}

/// Returns true when `nal_unit_type` is a non-VCL NAL unit type.
///
/// Native references:
/// - Declaration: `include/h265_common.h:190` (`IsNalUnitTypeNonVcl`)
/// - Unit test: `test/h265_common_unittest.cc:34`
pub fn common_is_nal_unit_type_non_vcl(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(nal_unit_type, ffi::h265nal_common_is_nal_unit_type_non_vcl)
}

/// Returns true when `nal_unit_type` is in the unspecified range.
///
/// Native references:
/// - Declaration: `include/h265_common.h:192` (`IsNalUnitTypeUnspecified`)
/// - Unit test: `test/h265_common_unittest.cc:40`
pub fn common_is_nal_unit_type_unspecified(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(
        nal_unit_type,
        ffi::h265nal_common_is_nal_unit_type_unspecified,
    )
}

/// Evaluates `more_rbsp_data` using an explicit cursor and returns both the
/// boolean result and post-call cursor offsets.
///
/// Native references:
/// - Declaration: `include/h265_common.h:207`
/// - Unit test: `test/h265_common_unittest.cc:129`
pub fn common_more_rbsp_data(
    data: &[u8],
    byte_offset: usize,
    bit_offset: usize,
) -> Result<MoreRbspDataResult, Error> {
    let mut out_has_more = 0u32;
    let mut out_byte_offset = 0usize;
    let mut out_bit_offset = 0usize;
    let status = unsafe {
        ffi::h265nal_common_more_rbsp_data(
            data.as_ptr(),
            data.len(),
            byte_offset,
            bit_offset,
            &mut out_has_more,
            &mut out_byte_offset,
            &mut out_bit_offset,
        )
    };
    status_to_result(status)?;
    Ok(MoreRbspDataResult {
        has_more_data: out_has_more != 0,
        byte_offset: out_byte_offset,
        bit_offset: out_bit_offset,
    })
}

fn call_nal_unit_type_predicate(
    nal_unit_type: u32,
    predicate: NalUnitTypePredicate,
) -> Result<bool, Error> {
    let mut out_value = 0u32;
    let status = unsafe { predicate(nal_unit_type, &mut out_value) };
    status_to_result(status)?;
    Ok(out_value != 0)
}
