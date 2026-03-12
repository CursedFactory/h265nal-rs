pub fn count_nalus_with_options(
    data: &[u8],
    nalu_length_bytes: i32,
    state: Option<&mut h265nal_sys::BitstreamParserState>,
) -> Result<usize, String> {
    let nals = if nalu_length_bytes < 0 {
        h265nal_sys::bitstream_parse(data, state, false)
    } else {
        h265nal_sys::bitstream_parse_nalu_length(data, nalu_length_bytes as usize, state, false)
    }
    .map_err(|err| format!("parse failed: {err}"))?;
    Ok(nals.len())
}

pub fn dump_nalus_annexb_to_stdout(
    data: &[u8],
    one_line: bool,
    add_offset: bool,
    add_length: bool,
    add_parsed_length: bool,
    add_checksum: bool,
    add_resolution: bool,
) -> Result<(), String> {
    let mut flags = 0u32;
    if one_line {
        flags |= h265nal_sys::DUMP_FLAG_ONE_LINE;
    }
    if add_offset {
        flags |= h265nal_sys::DUMP_FLAG_ADD_OFFSET;
    }
    if add_length {
        flags |= h265nal_sys::DUMP_FLAG_ADD_LENGTH;
    }
    if add_parsed_length {
        flags |= h265nal_sys::DUMP_FLAG_ADD_PARSED_LENGTH;
    }
    if add_checksum {
        flags |= h265nal_sys::DUMP_FLAG_ADD_CHECKSUM;
    }
    if add_resolution {
        flags |= h265nal_sys::DUMP_FLAG_ADD_RESOLUTION;
    }
    h265nal_sys::dump_annexb_to_stdout(data, flags).map_err(|err| format!("dump failed: {err}"))
}

pub fn parse_configuration_box(
    data: &[u8],
    state: &mut h265nal_sys::BitstreamParserState,
) -> Result<h265nal_sys::ConfigurationBoxFields, String> {
    h265nal_sys::configuration_box_parse(data, state)
        .map_err(|err| format!("hvcC parse failed: {err}"))
}

pub fn parse_bitstream(
    data: &[u8],
    nalu_length_bytes: i32,
    state: Option<&mut h265nal_sys::BitstreamParserState>,
    add_checksum: bool,
) -> Result<Vec<h265nal_sys::BitstreamNalFields>, String> {
    if nalu_length_bytes < 0 {
        return h265nal_sys::bitstream_parse(data, state, add_checksum)
            .map_err(|err| format!("parse failed: {err}"));
    }

    h265nal_sys::bitstream_parse_nalu_length(data, nalu_length_bytes as usize, state, add_checksum)
        .map_err(|err| format!("parse failed: {err}"))
}

pub fn nal_unit_type_to_string(nal_unit_type: u32) -> &'static str {
    match nal_unit_type {
        0 => "TRAIL_N",
        1 => "TRAIL_R",
        2 => "TSA_N",
        3 => "TSA_R",
        4 => "STSA_N",
        5 => "STSA_R",
        6 => "RADL_N",
        7 => "RADL_R",
        8 => "RASL_N",
        9 => "RASL_R",
        16 => "BLA_W_LP",
        17 => "BLA_W_RADL",
        18 => "BLA_N_LP",
        19 => "IDR_W_RADL",
        20 => "IDR_N_LP",
        21 => "CRA_NUT",
        32 => "VPS_NUT",
        33 => "SPS_NUT",
        34 => "PPS_NUT",
        35 => "AUD_NUT",
        36 => "EOS_NUT",
        37 => "EOB_NUT",
        38 => "FD_NUT",
        39 => "PREFIX_SEI_NUT",
        40 => "SUFFIX_SEI_NUT",
        _ => "UNKNOWN_NUT",
    }
}
