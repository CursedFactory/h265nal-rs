use std::env;
use std::process;

use h265nal_parity::run_compare;

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

    if command != "compare" {
        return Err(format!("unknown command `{command}`\n{}", usage()));
    }

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

fn usage() -> String {
    "Usage: h265nal-parity compare --scenario <name> [--fixture <path>]".to_string()
}
