use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

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

#[test]
fn infile_dash_reads_from_stdin() {
    let fixture =
        fs::read(fixture_path("pps_fdump_crash.202203.265")).expect("failed to read fixture file");

    let mut child = Command::new(env!("CARGO_BIN_EXE_h265nal-cli"))
        .arg("--output-format")
        .arg("json")
        .arg("--infile")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn h265nal-cli with stdin");

    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(&fixture)
        .expect("failed to write fixture bytes to stdin");

    let output = child
        .wait_with_output()
        .expect("failed waiting for h265nal-cli process");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let payload: serde_json::Value =
        serde_json::from_str(&stdout).expect("stdout should be valid json");
    assert!(payload["nal_units"].as_u64().unwrap_or(0) > 0);
}
