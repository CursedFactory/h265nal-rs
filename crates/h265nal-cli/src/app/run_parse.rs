use std::fs;
use std::io::Read;
use std::path::Path;

use crate::cli::output_format::OutputFormat;
use crate::config::runtime_options::RuntimeOptions;
use crate::ffi::parser;
use crate::render;

pub fn run(options: RuntimeOptions) -> Result<(), String> {
    validate_supported_options(&options)?;
    let mut parser_state = h265nal_sys::BitstreamParserState::new()
        .map_err(|err| format!("failed to create parser state: {err}"))?;

    if let Some(hvcc_path) = options.hvcc_file.as_deref() {
        let hvcc_bytes = read_input_bytes(hvcc_path)?;
        parser::parse_configuration_box(&hvcc_bytes, &mut parser_state)?;
    }

    let data = match options.input_path.as_deref() {
        Some(path) => Some(read_input_bytes(path)?),
        None => None,
    };

    match options.output_format {
        OutputFormat::C => run_c_mode(&options, data.as_deref(), &mut parser_state),
        OutputFormat::Json => run_json_mode(&options, data.as_deref(), &mut parser_state),
        OutputFormat::Pretty => run_pretty_mode(&options, data.as_deref(), &mut parser_state),
    }
}

fn validate_supported_options(options: &RuntimeOptions) -> Result<(), String> {
    if options.nalu_length_bytes < -1 {
        return Err("--nalu-length-bytes must be -1, 0, or a positive integer".to_string());
    }

    Ok(())
}

fn read_input_bytes(input_path: &Path) -> Result<Vec<u8>, String> {
    if input_path == Path::new("-") {
        let mut data = Vec::new();
        std::io::stdin()
            .read_to_end(&mut data)
            .map_err(|err| format!("failed to read stdin: {err}"))?;
        return Ok(data);
    }

    fs::read(input_path).map_err(|err| format!("failed to read {}: {err}", input_path.display()))
}

fn run_c_mode(
    options: &RuntimeOptions,
    data: Option<&[u8]>,
    parser_state: &mut h265nal_sys::BitstreamParserState,
) -> Result<(), String> {
    if options.dump_length {
        let data = data.ok_or_else(|| {
            "--dump-length requires an input bitstream (provide --infile or positional input)"
                .to_string()
        })?;
        return run_c_dump_length_mode(options, data, parser_state);
    }

    if options.dump_all {
        let data = data.ok_or_else(|| {
            "--dump-all requires an input bitstream (provide --infile or positional input)"
                .to_string()
        })?;
        if options.nalu_length_bytes != -1 {
            return Err(
                "--dump-all with --nalu-length-bytes is not implemented yet in Rust CLI"
                    .to_string(),
            );
        }
        if options.hvcc_file.is_some() {
            return Err(
                "--dump-all with --hvcc-file is not implemented yet in Rust CLI".to_string(),
            );
        }
        if matches!(options.outfile.as_deref(), Some(path) if path != Path::new("-")) {
            return Err("--outfile is not supported yet with --dump-all".to_string());
        }
        return parser::dump_nalus_annexb_to_stdout(data, options.as_one_line);
    }

    let count = match data {
        Some(data) => {
            parser::count_nalus_with_options(data, options.nalu_length_bytes, Some(parser_state))?
        }
        None => 0,
    };
    let output = render::c_output::render_nalu_count(count);
    write_output(&output, options.outfile.as_deref())
}

fn run_c_dump_length_mode(
    options: &RuntimeOptions,
    data: &[u8],
    parser_state: &mut h265nal_sys::BitstreamParserState,
) -> Result<(), String> {
    let nals = parser::parse_bitstream(
        data,
        options.nalu_length_bytes,
        Some(parser_state),
        options.add_checksum,
    )?;

    let fps = options.frames_per_second.unwrap_or(30.0);
    let mut rows = Vec::with_capacity(nals.len());
    let mut frame_rows = Vec::new();

    let mut frame_num = 0usize;
    let mut total_bytes = 0usize;
    let mut last_slice_nal_unit_type = 0u32;

    for (nal_num, nal) in nals.iter().enumerate() {
        let is_slice_segment = h265nal_sys::common_is_slice_segment(nal.nal_unit_type)
            .map_err(|err| format!("slice-segment check failed: {err}"))?;

        if is_slice_segment && nal.has_slice_segment_header {
            if nal.first_slice_segment_in_pic_flag == 1 && total_bytes > 0 {
                let bitrate_bps = ((total_bytes as f64) * 8.0 * fps) as usize;
                frame_rows.push(render::c_output::DumpLengthFrameRow {
                    frame_num,
                    nal_unit_type: last_slice_nal_unit_type,
                    bitrate_bps,
                });
                frame_num += 1;
                total_bytes = 0;
            }
            last_slice_nal_unit_type = nal.nal_unit_type;
            total_bytes += nal.length;
        }

        rows.push(render::c_output::DumpLengthRow {
            nal_num,
            frame_num,
            nal_unit_type: nal.nal_unit_type,
            nal_unit_type_str: parser::nal_unit_type_to_string(nal.nal_unit_type),
            nal_length_bytes: nal.length,
            first_slice_segment_in_pic_flag: if is_slice_segment && nal.has_slice_segment_header {
                Some(nal.first_slice_segment_in_pic_flag)
            } else {
                None
            },
            slice_segment_address: if is_slice_segment && nal.has_slice_segment_header {
                Some(nal.slice_segment_address)
            } else {
                None
            },
            slice_pic_order_cnt_lsb: if is_slice_segment && nal.has_slice_segment_header {
                Some(nal.slice_pic_order_cnt_lsb)
            } else {
                None
            },
        });
    }

    if total_bytes > 0 {
        let bitrate_bps = ((total_bytes as f64) * 8.0 * fps) as usize;
        frame_rows.push(render::c_output::DumpLengthFrameRow {
            frame_num,
            nal_unit_type: last_slice_nal_unit_type,
            bitrate_bps,
        });
    }

    let output = render::c_output::render_dump_length(&rows, &frame_rows);
    write_output(&output, options.outfile.as_deref())
}

fn write_output(output: &str, outfile: Option<&Path>) -> Result<(), String> {
    match outfile {
        Some(path) if path != Path::new("-") => fs::write(path, output)
            .map_err(|err| format!("failed to write {}: {err}", path.display())),
        _ => {
            print!("{output}");
            Ok(())
        }
    }
}

fn run_json_mode(
    options: &RuntimeOptions,
    data: Option<&[u8]>,
    parser_state: &mut h265nal_sys::BitstreamParserState,
) -> Result<(), String> {
    if options.dump_all {
        return Err("--dump-all is currently only supported with --output-format c".to_string());
    }

    let count = match data {
        Some(data) => {
            parser::count_nalus_with_options(data, options.nalu_length_bytes, Some(parser_state))?
        }
        None => 0,
    };
    let output = render::json_output::render_count(options, count)?;
    write_output(&output, options.outfile.as_deref())
}

fn run_pretty_mode(
    options: &RuntimeOptions,
    data: Option<&[u8]>,
    parser_state: &mut h265nal_sys::BitstreamParserState,
) -> Result<(), String> {
    if options.dump_all {
        return Err("--dump-all is currently only supported with --output-format c".to_string());
    }

    let count = match data {
        Some(data) => {
            parser::count_nalus_with_options(data, options.nalu_length_bytes, Some(parser_state))?
        }
        None => 0,
    };
    let output = render::pretty_output::render_count(options, count);
    write_output(&output, options.outfile.as_deref())
}
