use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawNalUnitFields};
use crate::state::BitstreamParserState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NalUnitFields {
    pub checksum_size: usize,
    pub checksum: [u8; 4],
    pub parsed_length: usize,
    pub forbidden_zero_bit: u32,
    pub nal_unit_type: u32,
    pub nuh_layer_id: u32,
    pub nuh_temporal_id_plus1: u32,
}

/// Returns NAL unit type for the provided buffer.
///
/// Native references:
/// - Declaration: `include/h265_nal_unit_header_parser.h:45` (`GetNalUnitType`)
/// - Unit test: `test/h265_nal_unit_parser_unittest.cc:143`
pub fn nal_unit_header_get_nal_unit_type(data: &[u8]) -> Result<u32, Error> {
    let mut out_nal_unit_type = 0u32;
    let status = unsafe {
        ffi::h265nal_nal_unit_header_get_nal_unit_type(
            data.as_ptr(),
            data.len(),
            &mut out_nal_unit_type,
        )
    };
    status_to_result(status)?;
    Ok(out_nal_unit_type)
}

/// Parses an escaped NAL unit and returns core parsed/header fields.
///
/// Native references:
/// - Declaration: `include/h265_nal_unit_parser.h:62` (`ParseNalUnit`)
/// - Unit tests: `test/h265_nal_unit_parser_unittest.cc:22`
/// - Unit tests: `test/h265_nal_unit_parser_unittest.cc:126`
pub fn nal_unit_parse(
    data: &[u8],
    state: &mut BitstreamParserState,
    add_checksum: bool,
) -> Result<NalUnitFields, Error> {
    let mut raw = RawNalUnitFields {
        checksum_size: 0,
        checksum: [0; 4],
        parsed_length: 0,
        forbidden_zero_bit: 0,
        nal_unit_type: 0,
        nuh_layer_id: 0,
        nuh_temporal_id_plus1: 0,
    };
    let status = unsafe {
        ffi::h265nal_nal_unit_parse(
            data.as_ptr(),
            data.len(),
            state.as_mut_ptr(),
            u32::from(add_checksum),
            &mut raw,
        )
    };
    status_to_result(status)?;
    Ok(NalUnitFields {
        checksum_size: raw.checksum_size,
        checksum: raw.checksum,
        parsed_length: raw.parsed_length,
        forbidden_zero_bit: raw.forbidden_zero_bit,
        nal_unit_type: raw.nal_unit_type,
        nuh_layer_id: raw.nuh_layer_id,
        nuh_temporal_id_plus1: raw.nuh_temporal_id_plus1,
    })
}
