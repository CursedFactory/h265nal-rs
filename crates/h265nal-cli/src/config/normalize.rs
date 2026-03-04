use crate::cli::args::CliArgs;

use super::runtime_options::RuntimeOptions;

pub fn normalize(args: CliArgs) -> Result<RuntimeOptions, String> {
    let input_path = resolve_input_path(&args)?;
    let mut add_offset = resolve_toggle(args.add_offset, args.no_add_offset, false);
    let mut add_length = resolve_toggle(args.add_length, args.no_add_length, false);
    let add_parsed_length =
        resolve_toggle(args.add_parsed_length, args.no_add_parsed_length, false);
    let add_checksum = resolve_toggle(args.add_checksum, args.no_add_checksum, false);
    let add_resolution = resolve_toggle(args.add_resolution, args.no_add_resolution, false);
    let add_contents = resolve_toggle(args.add_contents, args.no_add_contents, false);

    if add_contents {
        add_offset = true;
        add_length = true;
    }

    if args.dump_length {
        add_length = true;
    }

    Ok(RuntimeOptions {
        dump_all: args.dump_all,
        dump_length: args.dump_length,
        nalu_length_bytes: args.nalu_length_bytes,
        frames_per_second: args.frames_per_second,
        as_one_line: resolve_toggle(args.as_one_line, args.no_as_one_line, false),
        add_offset,
        add_length,
        add_parsed_length,
        add_checksum,
        add_resolution,
        add_contents,
        debug: args.debug,
        quiet: args.quiet,
        output_format: args.output_format,
        input_path,
        hvcc_file: args.hvcc_file,
        outfile: args.outfile,
    })
}

fn resolve_input_path(args: &CliArgs) -> Result<std::path::PathBuf, String> {
    if let (Some(infile), Some(positional)) = (&args.infile, &args.input_path) {
        if infile != positional {
            return Err(
                "both --infile and positional input were provided; use one source path".to_string(),
            );
        }
    }

    if let Some(infile) = &args.infile {
        return Ok(infile.clone());
    }

    if let Some(positional) = &args.input_path {
        return Ok(positional.clone());
    }

    if args.hvcc_file.is_some() {
        return Err(
            "hvcc-only input is not implemented yet; also provide --infile or positional input"
                .to_string(),
        );
    }

    Err("missing input: provide <input.265>, --infile, or --hvcc-file".to_string())
}

fn resolve_toggle(enabled: bool, disabled: bool, default_value: bool) -> bool {
    if disabled {
        return false;
    }
    if enabled {
        return true;
    }
    default_value
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::cli::output_format::OutputFormat;

    use super::*;

    fn base_args() -> CliArgs {
        CliArgs {
            infile: None,
            outfile: None,
            hvcc_file: None,
            dump_all: false,
            dump_length: false,
            nalu_length_bytes: -1,
            frames_per_second: None,
            as_one_line: true,
            no_as_one_line: false,
            add_offset: false,
            no_add_offset: false,
            add_length: false,
            no_add_length: false,
            add_parsed_length: false,
            no_add_parsed_length: false,
            add_checksum: false,
            no_add_checksum: false,
            add_resolution: false,
            no_add_resolution: false,
            add_contents: false,
            no_add_contents: false,
            debug: false,
            quiet: false,
            output_format: OutputFormat::C,
            input_path: Some(PathBuf::from("media/nvenc.265")),
        }
    }

    #[test]
    fn add_contents_forces_add_offset_and_add_length() {
        let mut args = base_args();
        args.add_contents = true;
        let options = normalize(args).expect("normalize should succeed");
        assert!(options.add_contents);
        assert!(options.add_offset);
        assert!(options.add_length);
    }

    #[test]
    fn dump_length_forces_add_length_even_if_disabled_explicitly() {
        let mut args = base_args();
        args.dump_length = true;
        args.no_add_length = true;
        let options = normalize(args).expect("normalize should succeed");
        assert!(options.dump_length);
        assert!(options.add_length);
    }

    #[test]
    fn no_toggle_wins_over_positive_toggle() {
        let mut args = base_args();
        args.as_one_line = true;
        args.no_as_one_line = true;
        let options = normalize(args).expect("normalize should succeed");
        assert!(!options.as_one_line);
    }

    #[test]
    fn as_one_line_defaults_to_true() {
        let args = base_args();
        let options = normalize(args).expect("normalize should succeed");
        assert!(options.as_one_line);
    }

    #[test]
    fn normalize_fails_when_no_sources_are_provided() {
        let mut args = base_args();
        args.input_path = None;
        let err = normalize(args).expect_err("normalize should fail");
        assert!(err.contains("missing input"));
    }
}
