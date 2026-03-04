use serde::Serialize;

use crate::config::runtime_options::RuntimeOptions;

#[derive(Serialize)]
pub struct JsonOutput {
    pub source: String,
    pub nal_units: usize,
    pub options: JsonOptions,
}

#[derive(Serialize)]
pub struct JsonOptions {
    pub dump_all: bool,
    pub dump_length: bool,
    pub as_one_line: bool,
    pub add_offset: bool,
    pub add_length: bool,
    pub add_parsed_length: bool,
    pub add_checksum: bool,
    pub add_resolution: bool,
    pub add_contents: bool,
}

pub fn render_count(options: &RuntimeOptions, count: usize) -> Result<String, String> {
    let payload = JsonOutput {
        source: options
            .input_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "(none)".to_string()),
        nal_units: count,
        options: JsonOptions {
            dump_all: options.dump_all,
            dump_length: options.dump_length,
            as_one_line: options.as_one_line,
            add_offset: options.add_offset,
            add_length: options.add_length,
            add_parsed_length: options.add_parsed_length,
            add_checksum: options.add_checksum,
            add_resolution: options.add_resolution,
            add_contents: options.add_contents,
        },
    };
    let mut rendered = serde_json::to_string_pretty(&payload)
        .map_err(|err| format!("json render failed: {err}"))?;
    rendered.push('\n');
    Ok(rendered)
}
