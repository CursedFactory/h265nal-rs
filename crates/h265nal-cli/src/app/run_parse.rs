use std::fs;
use std::io::Read;
use std::path::Path;

use crate::cli::output_format::OutputFormat;
use crate::config::runtime_options::RuntimeOptions;
use crate::ffi::parser;
use crate::render;

pub fn run(options: RuntimeOptions) -> Result<(), String> {
    validate_supported_options(&options)?;
    let data = read_input_bytes(&options.input_path)?;

    match options.output_format {
        OutputFormat::C => run_c_mode(&options, &data),
        OutputFormat::Json => run_json_mode(&options, &data),
        OutputFormat::Pretty => run_pretty_mode(&options, &data),
    }
}

fn validate_supported_options(options: &RuntimeOptions) -> Result<(), String> {
    if options.dump_length {
        return Err("--dump-length is not implemented yet in Rust CLI".to_string());
    }

    if options.nalu_length_bytes != -1 {
        return Err("--nalu-length-bytes is not implemented yet in Rust CLI".to_string());
    }

    if options.frames_per_second.is_some() {
        return Err("--frames-per-second is not implemented yet in Rust CLI".to_string());
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

fn run_c_mode(options: &RuntimeOptions, data: &[u8]) -> Result<(), String> {
    if options.dump_all {
        if matches!(options.outfile.as_deref(), Some(path) if path != Path::new("-")) {
            return Err("--outfile is not supported yet with --dump-all".to_string());
        }
        return parser::dump_nalus_annexb_to_stdout(data, options.as_one_line);
    }

    let count = parser::count_nalus_annexb(data)?;
    let output = render::c_output::render_nalu_count(count);
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

fn run_json_mode(options: &RuntimeOptions, data: &[u8]) -> Result<(), String> {
    if options.dump_all {
        return Err("--dump-all is currently only supported with --output-format c".to_string());
    }

    let count = parser::count_nalus_annexb(data)?;
    let output = render::json_output::render_count(options, count)?;
    write_output(&output, options.outfile.as_deref())
}

fn run_pretty_mode(options: &RuntimeOptions, data: &[u8]) -> Result<(), String> {
    if options.dump_all {
        return Err("--dump-all is currently only supported with --output-format c".to_string());
    }

    let count = parser::count_nalus_annexb(data)?;
    let output = render::pretty_output::render_count(options, count);
    write_output(&output, options.outfile.as_deref())
}
