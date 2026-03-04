//! Rust FFI bindings for the upstream h265nal native library.

const H265NAL_STATUS_OK: i32 = 0;
const H265NAL_STATUS_INVALID_ARGUMENT: i32 = 1;
const H265NAL_STATUS_PARSE_FAILURE: i32 = 2;
const H265NAL_STATUS_DUMP_UNAVAILABLE: i32 = 3;

pub const ABI_VERSION: u32 = 1;
pub const DUMP_FLAG_ONE_LINE: u32 = 1u32 << 0;

type NalUnitTypePredicate = unsafe extern "C" fn(u32, *mut u32) -> i32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidArgument,
    ParseFailure,
    DumpUnavailable,
    UnknownStatus(i32),
}

impl Error {
    fn from_status(status: i32) -> Self {
        match status {
            H265NAL_STATUS_INVALID_ARGUMENT => Self::InvalidArgument,
            H265NAL_STATUS_PARSE_FAILURE => Self::ParseFailure,
            H265NAL_STATUS_DUMP_UNAVAILABLE => Self::DumpUnavailable,
            other => Self::UnknownStatus(other),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::ParseFailure => write!(f, "parse failure"),
            Self::DumpUnavailable => write!(f, "dump unavailable in this build"),
            Self::UnknownStatus(status) => write!(f, "unknown status code {status}"),
        }
    }
}

impl std::error::Error for Error {}

#[repr(C)]
pub struct RawBitstreamParserState {
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
        let status = unsafe { h265nal_bitstream_parser_state_create(&mut raw) };
        status_to_result(status)?;
        Ok(Self { raw })
    }

    fn as_mut_ptr(&mut self) -> *mut RawBitstreamParserState {
        self.raw
    }
}

impl Drop for BitstreamParserState {
    fn drop(&mut self) {
        if self.raw.is_null() {
            return;
        }
        let _ = unsafe { h265nal_bitstream_parser_state_free(self.raw) };
        self.raw = std::ptr::null_mut();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoreRbspDataResult {
    pub has_more_data: bool,
    pub byte_offset: usize,
    pub bit_offset: usize,
}

unsafe extern "C" {
    pub fn h265nal_annexb_count_nalus(data: *const u8, len: usize, out_count: *mut usize) -> i32;
    pub fn h265nal_annexb_dump(data: *const u8, len: usize, dump_flags: u32) -> i32;
    pub fn h265nal_bitstream_parser_state_create(
        out_state: *mut *mut RawBitstreamParserState,
    ) -> i32;
    pub fn h265nal_bitstream_parser_state_free(state: *mut RawBitstreamParserState) -> i32;
    pub fn h265nal_common_is_slice_segment(
        nal_unit_type: u32,
        out_is_slice_segment: *mut u32,
    ) -> i32;
    pub fn h265nal_common_is_nal_unit_type_vcl(nal_unit_type: u32, out_is_vcl: *mut u32) -> i32;
    pub fn h265nal_common_is_nal_unit_type_non_vcl(
        nal_unit_type: u32,
        out_is_non_vcl: *mut u32,
    ) -> i32;
    pub fn h265nal_common_is_nal_unit_type_unspecified(
        nal_unit_type: u32,
        out_is_unspecified: *mut u32,
    ) -> i32;
    pub fn h265nal_common_more_rbsp_data(
        data: *const u8,
        len: usize,
        byte_offset: usize,
        bit_offset: usize,
        out_has_more: *mut u32,
        out_byte_offset: *mut usize,
        out_bit_offset: *mut usize,
    ) -> i32;
    pub fn h265nal_aud_parse_pic_type(data: *const u8, len: usize, out_pic_type: *mut u32) -> i32;
    pub fn h265nal_utils_get_slice_qp_y(
        data: *const u8,
        len: usize,
        state: *mut RawBitstreamParserState,
        out_values: *mut i32,
        out_capacity: usize,
        out_count: *mut usize,
    ) -> i32;
    pub fn h265nal_abi_version() -> u32;
}

pub fn abi_version() -> u32 {
    unsafe { h265nal_abi_version() }
}

pub fn count_nalus_annexb(data: &[u8]) -> Result<usize, Error> {
    let mut out_count = 0usize;
    let status = unsafe { h265nal_annexb_count_nalus(data.as_ptr(), data.len(), &mut out_count) };
    status_to_result(status)?;
    Ok(out_count)
}

pub fn dump_annexb_to_stdout(data: &[u8], flags: u32) -> Result<(), Error> {
    let status = unsafe { h265nal_annexb_dump(data.as_ptr(), data.len(), flags) };
    status_to_result(status)
}

/// Returns true when `nal_unit_type` is a slice segment type.
///
/// Native references:
/// - Declaration: `include/h265_common.h:186` (`IsSliceSegment`)
/// - Unit test: `test/h265_common_unittest.cc:22`
pub fn common_is_slice_segment(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(nal_unit_type, h265nal_common_is_slice_segment)
}

/// Returns true when `nal_unit_type` is a VCL NAL unit type.
///
/// Native references:
/// - Declaration: `include/h265_common.h:188` (`IsNalUnitTypeVcl`)
/// - Unit test: `test/h265_common_unittest.cc:27`
pub fn common_is_nal_unit_type_vcl(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(nal_unit_type, h265nal_common_is_nal_unit_type_vcl)
}

/// Returns true when `nal_unit_type` is a non-VCL NAL unit type.
///
/// Native references:
/// - Declaration: `include/h265_common.h:190` (`IsNalUnitTypeNonVcl`)
/// - Unit test: `test/h265_common_unittest.cc:34`
pub fn common_is_nal_unit_type_non_vcl(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(nal_unit_type, h265nal_common_is_nal_unit_type_non_vcl)
}

/// Returns true when `nal_unit_type` is in the unspecified range.
///
/// Native references:
/// - Declaration: `include/h265_common.h:192` (`IsNalUnitTypeUnspecified`)
/// - Unit test: `test/h265_common_unittest.cc:40`
pub fn common_is_nal_unit_type_unspecified(nal_unit_type: u32) -> Result<bool, Error> {
    call_nal_unit_type_predicate(nal_unit_type, h265nal_common_is_nal_unit_type_unspecified)
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
        h265nal_common_more_rbsp_data(
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

/// Parses an AUD NAL unit and returns `pic_type`.
///
/// Native references:
/// - Declaration: `include/h265_aud_parser.h:37` (`H265AudParser::ParseAud`)
/// - Unit test: `test/h265_aud_parser_unittest.cc:21`
pub fn aud_parse_pic_type(data: &[u8]) -> Result<u32, Error> {
    let mut out_pic_type = 0u32;
    let status =
        unsafe { h265nal_aud_parse_pic_type(data.as_ptr(), data.len(), &mut out_pic_type) };
    status_to_result(status)?;
    Ok(out_pic_type)
}

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
        h265nal_utils_get_slice_qp_y(
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
        h265nal_utils_get_slice_qp_y(
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

fn status_to_result(status: i32) -> Result<(), Error> {
    if status == H265NAL_STATUS_OK {
        Ok(())
    } else {
        Err(Error::from_status(status))
    }
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
