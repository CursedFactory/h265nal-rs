use crate::constants::H265NAL_STATUS_INVALID_ARGUMENT;
use crate::error::{status_to_result, Error};
use crate::ffi;
use crate::state::BitstreamParserState;

/// Extracts slice QP-Y values while reusing parser state across calls.
///
/// Native references:
/// - Declaration: `include/h265_utils.h:29`
/// - Unit test: `test/h265_bitstream_parser_unittest.cc:413`
pub fn utils_get_slice_qp_y(
    data: &[u8],
    state: &mut BitstreamParserState,
) -> Result<Vec<i32>, Error> {
    let mut out_count = 0usize;
    let status = unsafe {
        ffi::h265nal_utils_get_slice_qp_y(
            data.as_ptr(),
            data.len(),
            state.as_mut_ptr(),
            std::ptr::null_mut(),
            0,
            &mut out_count,
        )
    };

    if status != H265NAL_STATUS_INVALID_ARGUMENT {
        status_to_result(status)?;
    }

    let mut out_values = vec![0i32; out_count];
    let status = unsafe {
        ffi::h265nal_utils_get_slice_qp_y(
            data.as_ptr(),
            data.len(),
            state.as_mut_ptr(),
            out_values.as_mut_ptr(),
            out_values.len(),
            &mut out_count,
        )
    };
    status_to_result(status)?;
    out_values.truncate(out_count);
    Ok(out_values)
}
