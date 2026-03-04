use std::process::Command;

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
