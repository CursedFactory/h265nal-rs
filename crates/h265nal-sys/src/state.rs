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
