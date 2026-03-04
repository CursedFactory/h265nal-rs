pub mod diff;
pub mod normalize;
pub mod runner;
pub mod scenario;

use diff::compare_bytes;
use normalize::normalize_for_compare;
use runner::{run_baseline, run_local, RunOutput};
use scenario::{find_scenario, Scenario};

pub fn run_compare(scenario_name: &str, fixture_override: Option<&str>) -> Result<(), String> {
    let scenario = find_scenario(scenario_name)
        .ok_or_else(|| format!("unknown scenario `{scenario_name}`"))?;
    run_compare_scenario(scenario, fixture_override)
}

pub fn run_compare_scenario(
    scenario: Scenario,
    fixture_override: Option<&str>,
) -> Result<(), String> {
    let fixture = fixture_override.unwrap_or(scenario.default_fixture);
    let baseline_args = scenario.baseline_args(fixture);
    let local_args = scenario.local_args(fixture);

    let baseline_output = run_baseline(&baseline_args)?;
    let local_output = run_local(&local_args)?;

    assert_parity(scenario.name(), fixture, &baseline_output, &local_output)
}

fn assert_parity(
    scenario_name: &str,
    fixture: &str,
    baseline_output: &RunOutput,
    local_output: &RunOutput,
) -> Result<(), String> {
    let baseline_stdout = normalize_for_compare(&baseline_output.stdout);
    let local_stdout = normalize_for_compare(&local_output.stdout);
    let baseline_stderr = normalize_for_compare(&baseline_output.stderr);
    let local_stderr = normalize_for_compare(&local_output.stderr);

    let status_match = baseline_output.status_code == local_output.status_code;
    let stdout_mismatch = compare_bytes("stdout", baseline_stdout.as_ref(), local_stdout.as_ref());
    let stderr_mismatch = compare_bytes("stderr", baseline_stderr.as_ref(), local_stderr.as_ref());

    if status_match && stdout_mismatch.is_none() && stderr_mismatch.is_none() {
        return Ok(());
    }

    let mut diagnostic = String::new();
    diagnostic.push_str(&format!(
        "parity mismatch for scenario `{scenario_name}` (fixture `{fixture}`)"
    ));
    diagnostic.push_str(&format!("\nbaseline command: {}", baseline_output.command));
    diagnostic.push_str(&format!("\nlocal command: {}", local_output.command));

    if !status_match {
        diagnostic.push_str(&format!(
            "\nexit status mismatch: baseline={} local={}",
            baseline_output.status_code, local_output.status_code
        ));
    }

    for mismatch in [stdout_mismatch, stderr_mismatch].into_iter().flatten() {
        diagnostic.push_str(&format!(
            "\n{} bytes mismatch: baseline={} local={}",
            mismatch.stream_name, mismatch.baseline_len, mismatch.local_len
        ));
        diagnostic.push_str("\n");
        diagnostic.push_str(&mismatch.diff);
    }

    Err(diagnostic)
}
