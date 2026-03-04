use std::path::PathBuf;
use std::process::Command;

fn fixture_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../media")
        .join(file_name)
}

#[test]
fn json_output_includes_count_and_normalized_flags() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--output-format")
        .arg("json")
        .arg("--add-contents")
        .arg("--no-add-offset")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run json output mode");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let payload: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout should be valid json");

    assert!(payload["nal_units"].as_u64().unwrap_or(0) > 0);
    assert_eq!(payload["options"]["add_contents"].as_bool(), Some(true));
    assert_eq!(payload["options"]["add_offset"].as_bool(), Some(true));
    assert_eq!(payload["options"]["add_length"].as_bool(), Some(true));
}

#[test]
fn pretty_output_includes_key_sections() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--output-format")
        .arg("pretty")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run pretty output mode");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Input:"));
    assert!(stdout.contains("NAL units:"));
}

#[test]
fn dump_all_is_rejected_for_json_mode() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--output-format")
        .arg("json")
        .arg("--dump-all")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run json dump-all mode");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--dump-all is currently only supported with --output-format c"));
}
