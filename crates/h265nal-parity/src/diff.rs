use similar::TextDiff;

#[derive(Debug)]
pub struct ByteMismatch {
    pub stream_name: &'static str,
    pub baseline_len: usize,
    pub local_len: usize,
    pub diff: String,
}

pub fn compare_bytes(
    stream_name: &'static str,
    baseline: &[u8],
    local: &[u8],
) -> Option<ByteMismatch> {
    if baseline == local {
        return None;
    }

    let baseline_text = String::from_utf8_lossy(baseline);
    let local_text = String::from_utf8_lossy(local);
    let diff = TextDiff::from_lines(baseline_text.as_ref(), local_text.as_ref())
        .unified_diff()
        .header("baseline", "local")
        .to_string();

    Some(ByteMismatch {
        stream_name,
        baseline_len: baseline.len(),
        local_len: local.len(),
        diff,
    })
}

#[cfg(test)]
mod tests {
    use super::compare_bytes;

    #[test]
    fn unified_diff_includes_expected_context() {
        let mismatch = compare_bytes("stdout", b"line one\nline two\n", b"line one\nline 2\n")
            .expect("expected a mismatch");

        assert!(mismatch.diff.contains("--- baseline"));
        assert!(mismatch.diff.contains("+++ local"));
        assert!(mismatch.diff.contains("-line two"));
        assert!(mismatch.diff.contains("+line 2"));
    }
}
