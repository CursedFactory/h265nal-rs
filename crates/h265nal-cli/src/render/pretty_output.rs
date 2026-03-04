use crate::config::runtime_options::RuntimeOptions;

pub fn render_count(options: &RuntimeOptions, count: usize) -> String {
    format!(
        "Input: {}\nNAL units: {}\nMode: count\nDump mode: {}\nOne-line: {}\n",
        options.input_path.display(),
        count,
        options.dump_all,
        options.as_one_line
    )
}
