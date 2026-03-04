pub fn count_nalus_annexb(data: &[u8]) -> Result<usize, String> {
    h265nal_sys::count_nalus_annexb(data).map_err(|err| format!("count failed: {err}"))
}

pub fn dump_nalus_annexb_to_stdout(data: &[u8], one_line: bool) -> Result<(), String> {
    let flags = if one_line {
        h265nal_sys::DUMP_FLAG_ONE_LINE
    } else {
        0
    };
    h265nal_sys::dump_annexb_to_stdout(data, flags).map_err(|err| format!("dump failed: {err}"))
}
