use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

use h265nal_parity::report::{render_markdown_report, run_matrix};
use h265nal_parity::run_compare;
use h265nal_parity::scenario::all_scenarios;

fn main() {
    if let Err(message) = run_cli() {
        eprintln!("{message}");
        process::exit(2);
    }
}

fn run_cli() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        return Err(usage());
    };

    if command == "--help" || command == "-h" {
        println!("{}", usage());
        return Ok(());
    }

    match command.as_str() {
        "compare" => run_compare_command(args),
        "compare-all" => run_compare_all_command(args),
        "report" => run_report_command(args),
        "list" => {
            run_list_command();
            Ok(())
        }
        _ => Err(format!("unknown command `{command}`\n{}", usage())),
    }
}

fn run_compare_command(args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut scenario_name: Option<String> = None;
    let mut fixture_override: Option<String> = None;

    let remaining: Vec<String> = args.collect();
    let mut index = 0;
    while index < remaining.len() {
        match remaining[index].as_str() {
            "--scenario" => {
                index += 1;
                if index >= remaining.len() {
                    return Err("missing value for `--scenario`".to_string());
                }
                scenario_name = Some(remaining[index].clone());
            }
            "--fixture" => {
                index += 1;
                if index >= remaining.len() {
                    return Err("missing value for `--fixture`".to_string());
                }
                fixture_override = Some(remaining[index].clone());
            }
            flag => {
                return Err(format!("unknown argument `{flag}`\n{}", usage()));
            }
        }
        index += 1;
    }

    let scenario_name = scenario_name.ok_or_else(|| "`--scenario` is required".to_string())?;
    run_compare(&scenario_name, fixture_override.as_deref())?;
    println!("scenario `{scenario_name}` parity check passed");
    Ok(())
}

fn run_compare_all_command(args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut fixture_override: Option<String> = None;

    let remaining: Vec<String> = args.collect();
    let mut index = 0;
    while index < remaining.len() {
        match remaining[index].as_str() {
            "--fixture" => {
                index += 1;
                if index >= remaining.len() {
                    return Err("missing value for `--fixture`".to_string());
                }
                fixture_override = Some(remaining[index].clone());
            }
            flag => {
                return Err(format!("unknown argument `{flag}`\n{}", usage()));
            }
        }
        index += 1;
    }

    let report = run_matrix(fixture_override.as_deref())?;
    let unexpected = report.unexpected_mismatches();
    let closed = report.closed_gaps();

    println!(
        "matrix: total={} must_match={} known_gap={} matched={}",
        report.total(),
        report.must_match_total(),
        report.known_gap_total(),
        report.matched_total()
    );

    if !closed.is_empty() {
        println!("closed-gap scenarios detected:");
        for entry in closed {
            println!("- {}", entry.scenario.name);
        }
    }

    if !unexpected.is_empty() {
        let mut message = String::new();
        message.push_str("unexpected parity regressions in must-match scenarios:");
        for entry in unexpected {
            message.push_str(&format!("\n- {}", entry.scenario.name));
        }
        return Err(message);
    }

    println!("all must-match scenarios passed");
    Ok(())
}

fn run_report_command(args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut fixture_override: Option<String> = None;
    let mut output_path: Option<PathBuf> = None;

    let remaining: Vec<String> = args.collect();
    let mut index = 0;
    while index < remaining.len() {
        match remaining[index].as_str() {
            "--fixture" => {
                index += 1;
                if index >= remaining.len() {
                    return Err("missing value for `--fixture`".to_string());
                }
                fixture_override = Some(remaining[index].clone());
            }
            "--output" => {
                index += 1;
                if index >= remaining.len() {
                    return Err("missing value for `--output`".to_string());
                }
                output_path = Some(PathBuf::from(&remaining[index]));
            }
            flag => {
                return Err(format!("unknown argument `{flag}`\n{}", usage()));
            }
        }
        index += 1;
    }

    let report = run_matrix(fixture_override.as_deref())?;
    let markdown = render_markdown_report(&report);

    if let Some(path) = output_path {
        fs::write(&path, markdown)
            .map_err(|error| format!("failed to write report to {}: {error}", path.display()))?;
        println!("wrote parity report to {}", path.display());
    } else {
        print!("{markdown}");
    }

    let unexpected = report.unexpected_mismatches();
    if unexpected.is_empty() {
        return Ok(());
    }

    let mut message = String::new();
    message.push_str("unexpected parity regressions in must-match scenarios:");
    for entry in unexpected {
        message.push_str(&format!("\n- {}", entry.scenario.name));
    }
    Err(message)
}

fn run_list_command() {
    for scenario in all_scenarios() {
        println!(
            "{} [{}] - {}",
            scenario.name,
            scenario.expectation.label(),
            scenario.description
        );
    }
}

fn usage() -> String {
    [
        "Usage:",
        "  h265nal-parity compare --scenario <name> [--fixture <path>]",
        "  h265nal-parity compare-all [--fixture <path>]",
        "  h265nal-parity report [--fixture <path>] [--output <path>]",
        "  h265nal-parity list",
    ]
    .join("\n")
}
