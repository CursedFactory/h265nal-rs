use std::path::PathBuf;
use std::process::Command;

fn fixture_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../media")
        .join(file_name)
}

#[test]
fn help_prints_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--help")
        .output()
        .expect("failed to run --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("--dump-all"));
}

#[test]
fn version_prints_package_version() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--version")
        .output()
        .expect("failed to run --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn missing_input_reports_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .output()
        .expect("failed to run without input");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing input"));
}

#[test]
fn dump_length_outputs_csv_header() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump-length")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run with --dump-length");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nal_num,frame_num,nal_unit_type"));
}

#[test]
fn nalu_length_bytes_minus_one_is_accepted() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--nalu-length-bytes")
        .arg("-1")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run with --nalu-length-bytes");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nal_units="));
}

#[test]
fn frames_per_second_is_used_with_dump_length() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump-length")
        .arg("--frames-per-second")
        .arg("30")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run with --frames-per-second");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nal_num,frame_num,nal_unit_type"));
}

#[test]
fn dump_all_no_add_resolution_omits_sps_width_height() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump-all")
        .arg("--output-format")
        .arg("c")
        .arg("--no-as-one-line")
        .arg("--no-add-resolution")
        .arg(fixture_path("nvenc.265"))
        .output()
        .expect("failed to run with --dump-all --no-add-resolution");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Parity context: nal-compare found persistent diffs against chemag/h265nal
    // where Rust output added SPS-only `width:`/`height:` lines in dump-all mode.
    assert!(!stdout.contains("\n      width:"));
    assert!(!stdout.contains("\n      height:"));
}

#[test]
fn dump_all_add_resolution_includes_sps_width_height() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump-all")
        .arg("--output-format")
        .arg("c")
        .arg("--no-as-one-line")
        .arg("--add-resolution")
        .arg(fixture_path("nvenc.265"))
        .output()
        .expect("failed to run with --dump-all --add-resolution");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\n      width:"));
    assert!(stdout.contains("\n      height:"));
}
