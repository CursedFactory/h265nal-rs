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
fn dump_length_reports_not_implemented_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump-length")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run with --dump-length");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--dump-length is not implemented yet in Rust CLI"));
}

#[test]
fn nalu_length_bytes_reports_not_implemented_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--nalu-length-bytes")
        .arg("4")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run with --nalu-length-bytes");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--nalu-length-bytes is not implemented yet in Rust CLI"));
}

#[test]
fn frames_per_second_reports_not_implemented_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--frames-per-second")
        .arg("30")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run with --frames-per-second");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("--frames-per-second is not implemented yet in Rust CLI"));
}
