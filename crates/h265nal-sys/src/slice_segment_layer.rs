use crate::error::{status_to_result, Error};
use crate::ffi::{self, RawSliceSegmentLayerFields};
use crate::state::BitstreamParserState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliceSegmentLayerFields {
    pub has_slice_segment_header: u32,
    pub nal_unit_type: u32,
    pub first_slice_segment_in_pic_flag: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub slice_pic_parameter_set_id: u32,
    pub slice_segment_address: u32,
    pub slice_type: u32,
    pub slice_sao_luma_flag: u32,
    pub slice_sao_chroma_flag: u32,
    pub slice_qp_delta: i32,
    pub num_entry_point_offsets: u32,
}

/// Parses slice_segment_layer and returns selected header fields.
///
/// Native references:
/// - Declaration: `include/h265_slice_parser.h:174` (`ParseSliceSegmentLayer`)
/// - Unit test: `test/h265_slice_parser_unittest.cc:26`
/// - Unit test: `test/h265_slice_parser_unittest.cc:77`
/// - Unit test: `test/h265_slice_parser_unittest.cc:128`
pub fn slice_segment_layer_parse(
    data: &[u8],
    nal_unit_type: u32,
    state: &mut BitstreamParserState,
) -> Result<SliceSegmentLayerFields, Error> {
    let mut raw = RawSliceSegmentLayerFields {
        has_slice_segment_header: 0,
        nal_unit_type: 0,
        first_slice_segment_in_pic_flag: 0,
        no_output_of_prior_pics_flag: 0,
        slice_pic_parameter_set_id: 0,
        slice_segment_address: 0,
        slice_type: 0,
        slice_sao_luma_flag: 0,
        slice_sao_chroma_flag: 0,
        slice_qp_delta: 0,
        num_entry_point_offsets: 0,
    };
    let status = unsafe {
        ffi::h265nal_slice_segment_layer_parse(
            data.as_ptr(),
            data.len(),
            nal_unit_type,
            state.as_mut_ptr(),
            &mut raw,
        )
    };
    status_to_result(status)?;

    Ok(SliceSegmentLayerFields {
        has_slice_segment_header: raw.has_slice_segment_header,
        nal_unit_type: raw.nal_unit_type,
        first_slice_segment_in_pic_flag: raw.first_slice_segment_in_pic_flag,
        no_output_of_prior_pics_flag: raw.no_output_of_prior_pics_flag,
        slice_pic_parameter_set_id: raw.slice_pic_parameter_set_id,
        slice_segment_address: raw.slice_segment_address,
        slice_type: raw.slice_type,
        slice_sao_luma_flag: raw.slice_sao_luma_flag,
        slice_sao_chroma_flag: raw.slice_sao_chroma_flag,
        slice_qp_delta: raw.slice_qp_delta,
        num_entry_point_offsets: raw.num_entry_point_offsets,
    })
}
