use crate::config::runtime_options::RuntimeOptions;

pub fn render_count(options: &RuntimeOptions, count: usize) -> String {
    let source = options
        .input_path
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "(none)".to_string());
    format!(
        "Input: {}\nNAL units: {}\nMode: count\nDump mode: {}\nOne-line: {}\n",
        source, count, options.dump_all, options.as_one_line
    )
}
