use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawBitstreamNalFields};
use crate::state::BitstreamParserState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitstreamNalFields {
    pub offset: usize,
    pub length: usize,
    pub parsed_length: usize,
    pub checksum_size: usize,
    pub checksum: [u8; 4],
    pub forbidden_zero_bit: u32,
    pub nal_unit_type: u32,
    pub nuh_layer_id: u32,
    pub nuh_temporal_id_plus1: u32,
}

/// Parses an Annex-B bitstream and returns flattened per-NAL fields.
///
/// Native references:
/// - Declaration: `include/h265_bitstream_parser.h:44` (`ParseBitstream`)
/// - Unit tests: `test/h265_bitstream_parser_unittest.cc:57`
/// - Unit tests: `test/h265_bitstream_parser_unittest.cc:135`
/// - Unit tests: `test/h265_bitstream_parser_unittest.cc:289`
pub fn bitstream_parse(
    data: &[u8],
    state: Option<&mut BitstreamParserState>,
    add_checksum: bool,
) -> Result<Vec<BitstreamNalFields>, Error> {
    let mut out_count = 0usize;
    let state_ptr = match state {
        Some(state) => state.as_mut_ptr(),
        None => std::ptr::null_mut(),
    };
    let status = unsafe {
        ffi::h265nal_bitstream_parse(
            data.as_ptr(),
            data.len(),
            state_ptr,
            u32::from(add_checksum),
            std::ptr::null_mut(),
            0,
            &mut out_count,
        )
    };
    status_to_result(status)?;

    let mut raw_nals = Vec::with_capacity(out_count);
    raw_nals.resize_with(out_count, || RawBitstreamNalFields {
        offset: 0,
        length: 0,
        parsed_length: 0,
        checksum_size: 0,
        checksum: [0; 4],
        forbidden_zero_bit: 0,
        nal_unit_type: 0,
        nuh_layer_id: 0,
        nuh_temporal_id_plus1: 0,
    });

    let status = unsafe {
        ffi::h265nal_bitstream_parse(
            data.as_ptr(),
            data.len(),
            state_ptr,
            u32::from(add_checksum),
            raw_nals.as_mut_ptr(),
            raw_nals.len(),
            &mut out_count,
        )
    };
    status_to_result(status)?;

    Ok(raw_nals
        .into_iter()
        .map(|raw| BitstreamNalFields {
            offset: raw.offset,
            length: raw.length,
            parsed_length: raw.parsed_length,
            checksum_size: raw.checksum_size,
            checksum: raw.checksum,
            forbidden_zero_bit: raw.forbidden_zero_bit,
            nal_unit_type: raw.nal_unit_type,
            nuh_layer_id: raw.nuh_layer_id,
            nuh_temporal_id_plus1: raw.nuh_temporal_id_plus1,
        })
        .collect())
}
