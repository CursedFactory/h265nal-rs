use std::path::PathBuf;
use std::process::Command;

fn fixture_path(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../media")
        .join(file_name)
}

#[test]
fn count_mode_prints_nal_unit_count() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run h265nal-cli");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nal_units="));
}

#[test]
fn dump_mode_prints_nal_units() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump")
        .arg(fixture_path("nvenc.265"))
        .output()
        .expect("failed to run h265nal-cli --dump");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nal_unit {"));
}

#[test]
fn dump_one_line_mode_prints_nal_units() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump")
        .arg("--one-line")
        .arg(fixture_path("nvenc.265"))
        .output()
        .expect("failed to run h265nal-cli --dump --one-line");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nal_unit {"));
}

#[test]
fn dump_regression_pps_fdump_crash_no_segfault() {
    let output = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--dump")
        .arg(fixture_path("pps_fdump_crash.202203.265"))
        .output()
        .expect("failed to run h265nal-cli --dump on crash fixture");

    assert!(
        output.status.success(),
        "dump should not crash on crash fixture"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("nal_unit {"));
}
