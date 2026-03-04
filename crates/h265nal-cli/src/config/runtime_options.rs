use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::cli::output_format::OutputFormat;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeOptions {
    pub dump_all: bool,
    pub dump_length: bool,
    pub nalu_length_bytes: i32,
    pub frames_per_second: Option<f64>,
    pub as_one_line: bool,
    pub add_offset: bool,
    pub add_length: bool,
    pub add_parsed_length: bool,
    pub add_checksum: bool,
    pub add_resolution: bool,
    pub add_contents: bool,
    pub debug: bool,
    pub quiet: bool,
    pub output_format: OutputFormat,
    pub input_path: PathBuf,
    pub hvcc_file: Option<PathBuf>,
    pub outfile: Option<PathBuf>,
}
