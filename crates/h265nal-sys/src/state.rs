use crate::error::{status_to_result, Error};
use crate::ffi;

#[repr(C)]
pub(crate) struct RawBitstreamParserState {
    _private: [u8; 0],
}

/// Opaque parser state used by stateful helpers.
///
/// Native references:
/// - Declaration: `include/h265_bitstream_parser_state.h:20`
/// - Unit tests: `test/h265_bitstream_parser_unittest.cc:289`
/// - Unit tests: `test/h265_bitstream_parser_unittest.cc:413`
pub struct BitstreamParserState {
    raw: *mut RawBitstreamParserState,
}

impl BitstreamParserState {
    pub fn new() -> Result<Self, Error> {
        let mut raw = std::ptr::null_mut();
        let status = unsafe { ffi::h265nal_bitstream_parser_state_create(&mut raw) };
        status_to_result(status)?;
        Ok(Self { raw })
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut RawBitstreamParserState {
        self.raw
    }

    /// Seeds a mock VPS entry in parser state for slice parity tests.
    ///
    /// Native references:
    /// - Unit test: `test/h265_slice_parser_unittest.cc:39`
    pub fn seed_vps(&mut self, vps_id: u32) -> Result<(), Error> {
        let status =
            unsafe { ffi::h265nal_bitstream_parser_state_seed_vps(self.as_mut_ptr(), vps_id) };
        status_to_result(status)
    }

    /// Seeds a mock SPS entry in parser state for slice parity tests.
    ///
    /// Native references:
    /// - Unit test: `test/h265_slice_parser_unittest.cc:41`
    /// - Unit test: `test/h265_slice_parser_unittest.cc:89`
    #[allow(clippy::too_many_arguments)]
    pub fn seed_sps(
        &mut self,
        sps_id: u32,
        sample_adaptive_offset_enabled_flag: u32,
        chroma_format_idc: u32,
        log2_min_luma_coding_block_size_minus3: u32,
        log2_diff_max_min_luma_coding_block_size: u32,
        pic_width_in_luma_samples: u32,
        pic_height_in_luma_samples: u32,
    ) -> Result<(), Error> {
        let status = unsafe {
            ffi::h265nal_bitstream_parser_state_seed_sps(
                self.as_mut_ptr(),
                sps_id,
                sample_adaptive_offset_enabled_flag,
                chroma_format_idc,
                log2_min_luma_coding_block_size_minus3,
                log2_diff_max_min_luma_coding_block_size,
                pic_width_in_luma_samples,
                pic_height_in_luma_samples,
            )
        };
        status_to_result(status)
    }

    /// Seeds a mock PPS entry in parser state for slice parity tests.
    ///
    /// Native references:
    /// - Unit test: `test/h265_slice_parser_unittest.cc:45`
    /// - Unit test: `test/h265_slice_parser_unittest.cc:165`
    pub fn seed_pps(&mut self, pps_id: u32) -> Result<(), Error> {
        let status =
            unsafe { ffi::h265nal_bitstream_parser_state_seed_pps(self.as_mut_ptr(), pps_id) };
        status_to_result(status)
    }
}

impl Drop for BitstreamParserState {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        let _ = unsafe { ffi::h265nal_bitstream_parser_state_free(self.raw) };
        self.raw = std::ptr::null_mut();
    }
}
