use std::path::PathBuf;

use clap::{ArgAction, Parser};

use crate::cli::output_format::OutputFormat;

#[derive(Clone, Debug, Parser)]
#[command(name = "h265nal-cli", about = "Rust CLI for h265nal", version)]
pub struct CliArgs {
    #[arg(short = 'i', long = "infile", value_name = "input.265")]
    pub infile: Option<PathBuf>,

    #[arg(short = 'o', long = "outfile", value_name = "output")]
    pub outfile: Option<PathBuf>,

    #[arg(long = "hvcc-file", value_name = "hvcc.bin")]
    pub hvcc_file: Option<PathBuf>,

    #[arg(long = "dump-all", visible_alias = "dump", action = ArgAction::SetTrue)]
    pub dump_all: bool,

    #[arg(long = "dump-length", action = ArgAction::SetTrue)]
    pub dump_length: bool,

    #[arg(long = "nalu-length-bytes", default_value_t = -1)]
    pub nalu_length_bytes: i32,

    #[arg(long = "frames-per-second", value_name = "fps")]
    pub frames_per_second: Option<f64>,

    #[arg(
        long = "as-one-line",
        visible_alias = "one-line",
        action = ArgAction::SetTrue,
        default_value_t = true
    )]
    pub as_one_line: bool,

    #[arg(long = "no-as-one-line", action = ArgAction::SetTrue)]
    pub no_as_one_line: bool,

    #[arg(long = "add-offset", action = ArgAction::SetTrue)]
    pub add_offset: bool,

    #[arg(long = "no-add-offset", action = ArgAction::SetTrue)]
    pub no_add_offset: bool,

    #[arg(long = "add-length", action = ArgAction::SetTrue)]
    pub add_length: bool,

    #[arg(long = "no-add-length", action = ArgAction::SetTrue)]
    pub no_add_length: bool,

    #[arg(long = "add-parsed-length", action = ArgAction::SetTrue)]
    pub add_parsed_length: bool,

    #[arg(long = "no-add-parsed-length", action = ArgAction::SetTrue)]
    pub no_add_parsed_length: bool,

    #[arg(long = "add-checksum", action = ArgAction::SetTrue)]
    pub add_checksum: bool,

    #[arg(long = "no-add-checksum", action = ArgAction::SetTrue)]
    pub no_add_checksum: bool,

    #[arg(long = "add-resolution", action = ArgAction::SetTrue)]
    pub add_resolution: bool,

    #[arg(long = "no-add-resolution", action = ArgAction::SetTrue)]
    pub no_add_resolution: bool,

    #[arg(long = "add-contents", action = ArgAction::SetTrue)]
    pub add_contents: bool,

    #[arg(long = "no-add-contents", action = ArgAction::SetTrue)]
    pub no_add_contents: bool,

    #[arg(short = 'd', long = "debug", action = ArgAction::SetTrue)]
    pub debug: bool,

    #[arg(long = "quiet", action = ArgAction::SetTrue)]
    pub quiet: bool,

    #[arg(long = "output-format", value_enum, default_value_t = OutputFormat::C)]
    pub output_format: OutputFormat,

    #[arg(value_name = "input.265")]
    pub input_path: Option<PathBuf>,
}
