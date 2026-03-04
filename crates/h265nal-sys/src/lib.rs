//! Rust FFI bindings for the upstream h265nal native library.

const H265NAL_STATUS_OK: i32 = 0;
const H265NAL_STATUS_INVALID_ARGUMENT: i32 = 1;
const H265NAL_STATUS_PARSE_FAILURE: i32 = 2;
const H265NAL_STATUS_DUMP_UNAVAILABLE: i32 = 3;

pub const ABI_VERSION: u32 = 1;
pub const DUMP_FLAG_ONE_LINE: u32 = 1u32 << 0;

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

unsafe extern "C" {
    pub fn h265nal_annexb_count_nalus(data: *const u8, len: usize, out_count: *mut usize) -> i32;
    pub fn h265nal_annexb_dump(data: *const u8, len: usize, dump_flags: u32) -> i32;
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

fn status_to_result(status: i32) -> Result<(), Error> {
    if status == H265NAL_STATUS_OK {
        Ok(())
    } else {
        Err(Error::from_status(status))
    }
}
