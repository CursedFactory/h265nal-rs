use crate::error::{status_to_result, Error};
use crate::ffi;

pub fn abi_version() -> u32 {
    unsafe { ffi::h265nal_abi_version() }
}

pub fn count_nalus_annexb(data: &[u8]) -> Result<usize, Error> {
    let mut out_count = 0usize;
    let status =
        unsafe { ffi::h265nal_annexb_count_nalus(data.as_ptr(), data.len(), &mut out_count) };
    status_to_result(status)?;
    Ok(out_count)
}

pub fn dump_annexb_to_stdout(data: &[u8], flags: u32) -> Result<(), Error> {
    let status = unsafe { ffi::h265nal_annexb_dump(data.as_ptr(), data.len(), flags) };
    status_to_result(status)
}
