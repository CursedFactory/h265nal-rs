use crate::error::{status_to_result, Error};
use crate::ffi;

/// Parses an AUD NAL unit and returns `pic_type`.
///
/// Native references:
/// - Declaration: `include/h265_aud_parser.h:37` (`H265AudParser::ParseAud`)
/// - Unit test: `test/h265_aud_parser_unittest.cc:21`
pub fn aud_parse_pic_type(data: &[u8]) -> Result<u32, Error> {
    let mut out_pic_type = 0u32;
    let status =
        unsafe { ffi::h265nal_aud_parse_pic_type(data.as_ptr(), data.len(), &mut out_pic_type) };
    status_to_result(status)?;
    Ok(out_pic_type)
}
