use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

#[derive(Clone, Debug)]
pub struct RunOutput {
    pub command: String,
    pub status_code: i32,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub fn run_baseline(args: &[String]) -> Result<RunOutput, String> {
    run_service("baseline", args)
}

pub fn run_local(args: &[String]) -> Result<RunOutput, String> {
    run_service("local", args)
}

pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crate should be nested under workspace root")
        .to_path_buf()
}

pub fn compose_file() -> PathBuf {
    repo_root().join("docker/compose.yml")
}

fn run_service(service: &str, args: &[String]) -> Result<RunOutput, String> {
    let compose_path = compose_file();
    ensure_compose_images_built(&compose_path)?;

    let mut command = Command::new("docker");
    command.arg("compose");
    command.arg("-f");
    command.arg(&compose_path);
    command.arg("run");
    command.arg("--rm");
    command.arg("-T");
    command.arg("--no-build");
    command.arg("--quiet-build");
    command.arg(service);
    command.arg("--");
    command.args(args);

    let rendered_command = render_command(&compose_path, service, args);
    let output = command
        .output()
        .map_err(|error| format!("failed to run `{rendered_command}`: {error}"))?;

    Ok(RunOutput {
        command: rendered_command,
        status_code: output.status.code().unwrap_or(-1),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

fn render_command(compose_path: &Path, service: &str, args: &[String]) -> String {
    let mut parts = vec![
        "docker".to_string(),
        "compose".to_string(),
        "-f".to_string(),
        compose_path.display().to_string(),
        "run".to_string(),
        "--rm".to_string(),
        "-T".to_string(),
        "--no-build".to_string(),
        "--quiet-build".to_string(),
        service.to_string(),
        "--".to_string(),
    ];
    parts.extend(args.iter().cloned());
    parts.join(" ")
}

fn ensure_compose_images_built(compose_path: &Path) -> Result<(), String> {
    static BUILD_RESULT: OnceLock<Result<(), String>> = OnceLock::new();
    BUILD_RESULT
        .get_or_init(|| build_compose_images(compose_path))
        .clone()
}

fn build_compose_images(compose_path: &Path) -> Result<(), String> {
    let output = Command::new("docker")
        .arg("compose")
        .arg("-f")
        .arg(compose_path)
        .arg("build")
        .arg("--quiet")
        .output()
        .map_err(|error| {
            format!(
                "failed to pre-build docker compose services for {}: {error}",
                compose_path.display()
            )
        })?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    Err(format!(
        "docker compose build failed (exit={}):\nstdout:\n{}\nstderr:\n{}",
        output.status.code().unwrap_or(-1),
        stdout,
        stderr
    ))
}
